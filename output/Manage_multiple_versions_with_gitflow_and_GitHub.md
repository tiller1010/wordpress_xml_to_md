created: 2022-09-04T21:46:01+00:00
tags: [actions]

# Manage multiple versions with gitflow and GitHub


Imagine this: you want to add a new feature to a repository, but it involves a change that will break backward compatibility. You could release a new major version, but other repositories may depend on the previous version of your software. In this case, you need to support an older version simultaneously with the next major version. Git-flow is a branching strategy that eases the burden of managing multiple versions, but it can be tricky to implement in a team using GitHub. Git-flow provides several benefits:

* Git-flow uses distinct and obvious branch roles: branches are annotated with their specific purpose (feature, release, support, and hotfix).
* Git-flow keeps consistent histories: patches made for older versions are automatically merged back into the development branch.
* Git-flow uses a release branching strategy that ensures a higher quality of released software, and lower stress for the developer managing branches, as release branches are deleted as soon as they are merged and tagged.

These benefits can be achieved in a GitHub setting, but it requires some manual automation with GitHub actions.

"feature/feature\_name" into "develop"
----------

All new features should be created from the "develop" branch. No GitHub actions need to run on pull requests created for feature branches into the develop branch, but you may choose to enforce labels on your features. Making sure the feature in review has the appropriate semantic versioning label will be helpful for when we create "release" branches later.

Draft a release on "release/\*" push
----------

By far the best benefit of using git-flow with GitHub is that releases can be properly reviewed in the form of a pull request. A GitHub action could be created that drafts a release when a push is made to a "release/\*" branch.

```
on:
  push:
    branches:
      - release/*

permissions:
  contents: read

jobs:
  update_release_draft:
    permissions:
      contents: write
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@master

      # Get the release tag
      - name: Get release Tag
        id: get_release_tag
        run: |
          RELEASETAG=$(git branch | grep \* | sed -re "s/release\///;s/\*//;s/\s*//g")
          echo "::set-output name=release_tag::$RELEASETAG"

      # Draft Release with release branch
      - name: Draft Release with release branch
        id: update_release_draft_with_release_branch
        uses: release-drafter/release-drafter@master
        with:
          tag: ${{ steps.get_release_tag.outputs.release_tag }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Publish the draft release when "release/\*" PRs are merged
----------

When a release branch is merged into "main" the release that was drafted for that pull request can be published. Doing so creates a tag. After the corresponding release is published, the "release/\*" branch can be automatically deleted by a GitHub action.

```
on:
  pull_request:
    types:
      - closed
    branches:
      - main

jobs:
  publish_release:
    if: ${{ github.event.pull_request.merged }}
    runs-on: ubuntu-latest
    steps:
      # Get the latest draft release. This requires authentication.
      - name: Get release id
        id: get_release_id
        run: |
          TOKEN=${{ secrets.GITHUB_TOKEN }}
          RELEASEID=$(curl -H "Accept: application/vnd.github+json" -H "Authorization: token $TOKEN" https://api.github.com/repos/[YOURNAME]/[REPONAME]/releases)
          RELEASEID=$(echo "$RELEASEID" | grep \"id\"  | head -n 1 | sed -re "s/[a-z]*//g;s/[-|,|:|'\"]//g;s/\s//g")
          echo "::set-output name=release_id::$RELEASEID"
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      # Publish the release draft
      - name: Publish release
        uses: eregon/publish-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          release_id: ${{ steps.get_release_id.outputs.release_id }}

      # Get payload info from the pull request
      - name: Payload info
        id: payload_info
        uses: Dovyski/payload-info-action@master
        continue-on-error: true

      # Remove the release or hotfix branch after publishing
      - name: Remove PR branch
        uses: dawidd6/action-delete-branch@v3
        with:
          github_token: ${{github.token}}
          branches: ${{ steps.payload_info.outputs.branch }}
```

Merge main into develop
----------

After a release branch is merged into the main branch, it should be merged back into the "develop" branch. Optionally, you could set GitHub actions to create a commit that updates a CHANGELOG.md file before this merge occurs.

```
on:
  push:
    branches:
      - main

permissions:
  contents: write

jobs:
  sync_develop:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@master

      - name: Merge main -> develop
        uses: devmasx/merge-branch@master
        with:
          type: now
          from_branch: main
          target_branch: develop
          github_token: ${{ secrets.GITHUB_TOKEN }}
          message: Merged main into develop
```

Draft releases on "hotfix/\*" push
----------

When you must support a previous minor or major version, you should start by checking out that tag. Make sure to checkout the latest patch for that minor or major version. Then, you should create a "support/\*.x" branch. Support branches should function similarly to the "main" branch. Tags will be created when "hotfix/\*" branches are merged into it, and commits should **not** be added to it directly. After creating the support branch, a "hotfix/\*" branch with the next patch number should be created. These are similar to the "release/\*" branches, as they should be reviewed in a pull request, and when they are merged, Git actions should **publish** the drafted release and delete the "hotfix/\*" branch (similar to when a "release/\*" branch is merged, see the above example).

```
on:
  push:
    branches:
      - hotfix/*

jobs:
  update_release_draft:
    permissions:
      contents: write
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@master

      # Get the hotfix tag
      - name: Get hotfix Tag
        id: get_hotfix_tag
        run: |
          RELEASETAG=$(git branch | grep \* | sed -re "s/hotfix\///;s/\*//;s/\s*//g")
          echo "::set-output name=release_tag::$RELEASETAG"

      # Draft Release with hotfix branch
      - name: Draft Release with hotfix branch
        id: update_release_draft_with_hotfix_branch
        uses: release-drafter/release-drafter@master
        with:
          tag: ${{ steps.get_hotfix_tag.outputs.release_tag }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Merge support into main and develop
----------

If a bug existed in a previous version, it is likely that it still exists in the latest version too. Similar to "release/\*" branches that are merged into the main branch, support branches too should be merged into the main branch and then merged again into the develop branch when a "hotfix/\*" branch is merged into one. There is, however, a higher potential for this merge to conflict with the main branch. In that case, a developer must resolve the conflict on their own, and push the resolved merge into the main branch.

```
on:
  push:
    branches:
      - support/*

permissions:
  contents: write

jobs:
  sync_develop:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@master

      # Get the support branch
      - name: Get support branch
        id: get_support_branch
        run: |
          SUPPORTBRANCH=$(git branch | grep \* | sed -re "s/[\*|\ ]//g")
          echo "::set-output name=support_branch::$SUPPORTBRANCH"

      - name: Merge support -> main
        uses: devmasx/merge-branch@master
        with:
          type: now
          from_branch: ${{ steps.get_support_branch.outputs.support_branch }}
          target_branch: main
          github_token: ${{ secrets.GITHUB_TOKEN }}
          message: Merged ${{ steps.get_support_branch.outputs.support_branch }} into main

      - name: Merge main -> develop
        uses: devmasx/merge-branch@master
        with:
          type: now
          from_branch: main
          target_branch: develop
          github_token: ${{ secrets.GITHUB_TOKEN }}
          message: Merged main into develop
```

I hope you decide to use git-flow in your projects! It adds much-needed sanity to the management of multiple versions of software.

