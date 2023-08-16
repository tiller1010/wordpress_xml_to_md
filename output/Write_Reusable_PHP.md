# Write Reusable PHP


Have you ever found yourself writing code with a sense Déjà vu? You feel as though the code you are writing has already been written somewhere else. This could potentially be within the same project or even the same file. You do not want to be stuck repeating yourself and you do not want to waste time finding answers to the problems you have already solved. You might already be familiar with the "DRY" principle (Don't Repeat Yourself). Repeating code not only makes your project look sloppy and difficult to read (and therefore manage); it increases the likelihood of bugs and makes the code more difficult to manage. Here are a few helpful tips for producing PHP code that you can use often, but write only once.

Use Modules
----------

Modules are reusable packages that can be included across several of your projects. Modules isolate the functionality of one of your project's features, allowing you to maintain these features separately from your projects. You might have several PHP-powered websites, most of which connect to social media platforms. Therefore, you might create a module just for managing your social media accounts. Such a module should not be overly concerned with the particular project it is included in but instead should only care about storing data related to social media sites. Perhaps the module would also provide some sort of front-end component to render this data such as icon links to each social network site. If an update were to be added to support some new feature (perhaps a selectable list of icons), then the update would be made to the module itself and not the websites. Any site that uses this module could receive the update by [running composer](https://getcomposer.org/).

Modules are typically namespaced, meaning their classes can only be used in a project when explicitly including it. For example, the full class name for social media links might look something like "Vendor\\SocialMedia\\SocialMediaLink". You could use this class directly by prepending the class with a backslash like:

```
$facebook = new \Vendor\SocialMedia\SocialMediaLink();
```

Or you could include the class with the "use" statement:

```
use Vendor\SocialMedia\SocialMediaLink; $facebook = new SocialMediaLink();
```

Namespacing your modules ensures that the class names never conflict with those that are defined globally.

Write Methods for common actions
----------

There are some processes that will need to occur in several areas of your project. This could be something like formatting dates. It is usually a good idea to keep a consistent date format across a project. Your code could have several functions to retrieve dates, but it could be repeating the same code that formats the date in each. It might make sense for you to write a method just for formatting the dates. With one reusable method for date formatting, you will not find yourself writing the same code in several areas.

Sticking to this idea, consider the purpose of each function before writing. If the function requires several separate processes to occur before the main goal is achieved, it would make sense to break each task out into a separate function. Doing this makes your code more readable and more reusable. For example, your code might have a method that retrieves and renders blog articles that have been marked "Published". We will call this method "renderPublishedArticles". If your website has a content management system, it would be a good idea to flag unpublished articles. The code here will likely use a method like "getAllArticles" to list articles in the CMS. Instead of repeating code to check if articles are published in both methods, a better solution would be to write a method that just checks the "Published" status of an article. This method might be named "getPublishedArticles". The "renderPublishedArticles" and "getAllArticles" methods would both call this method. Because the functionality has been encapsulated in a function name, we know exactly what is happening when reading the code for each method.

If a change has to be made, the change is made in one area of the code, rather than each time the code is repeated. In the case of our date formatting method, we might opt for a different date format. You will thank youself later for writing reusable methods when changes like this are necessary.

Use Classes to Encapsulate Components
----------

If you are familiar with Object-Oriented Programming (OOP), you know that you can use classes to encapsulate certain behaviors of your components. In object-oriented programming languages like PHP, you might have a class that represents some type of model. That model will be responsible for defining some data structure, and it will likely provide methods to interact with related components. For example, you might have a class called "Page". This model might define fillable database fields like "HTML Content" and "Featured Images". This class could have methods to interact with these relationships, such as a "getRecentFeaturedImages" method.

A website will commonly have different types of pages. Some functionality should be shared between all pages, but some pages will have different concerns. For example, a blog page might be responsible for storing and fetching written articles. A home page will usually feature a large banner image, video, or slideshow. If we want our "Page" class to be extendable, it should only be concerned with the functionality that should be shared across every page type. We could then extend the Page class for each page type and add the appropriate functionality:

```
class Page{ // Define the functionality that should exist on all pages ...}class HomePage extends Page{  // This inherits from "Page", so just define everything unique to a Home Page ...}
```

Conclusion
----------

Writing reusable code has a plethora of benefits, as you have seen. Employing these tips means writing code that is consistent everywhere, even across projects. When many common actions are abstracted into separate methods, the code will naturally become more readable and declarative, as most methods will call other methods, each with descriptive names. When functionality needs to be shared between components, having a shared base class that each extends means writing less code overall. While it might be easy to simply paste in some code that is already used somewhere else, it will be more difficult to maintain later on should things change. Instead, think about how to keep each bit of your code clear in its purpose, so that it is reusable, should you need it for something else tomorrow.



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
on:  push:    branches:      - release/*permissions:  contents: readjobs:  update_release_draft:    permissions:      contents: write    runs-on: ubuntu-latest    steps:      - name: Checkout        uses: actions/checkout@master      # Get the release tag      - name: Get release Tag        id: get_release_tag        run: |          RELEASETAG=$(git branch | grep \* | sed -re "s/release\///;s/\*//;s/\s*//g")          echo "::set-output name=release_tag::$RELEASETAG"      # Draft Release with release branch      - name: Draft Release with release branch        id: update_release_draft_with_release_branch        uses: release-drafter/release-drafter@master        with:          tag: ${{ steps.get_release_tag.outputs.release_tag }}        env:          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Publish the draft release when "release/\*" PRs are merged
----------

When a release branch is merged into "main" the release that was drafted for that pull request can be published. Doing so creates a tag. After the corresponding release is published, the "release/\*" branch can be automatically deleted by a GitHub action.

```
on:  pull_request:    types:      - closed    branches:      - mainjobs:  publish_release:    if: ${{ github.event.pull_request.merged }}    runs-on: ubuntu-latest    steps:      # Get the latest draft release. This requires authentication.      - name: Get release id        id: get_release_id        run: |          TOKEN=${{ secrets.GITHUB_TOKEN }}          RELEASEID=$(curl -H "Accept: application/vnd.github+json" -H "Authorization: token $TOKEN" https://api.github.com/repos/[YOURNAME]/[REPONAME]/releases)          RELEASEID=$(echo "$RELEASEID" | grep \"id\"  | head -n 1 | sed -re "s/[a-z]*//g;s/[-|,|:|'\"]//g;s/\s//g")          echo "::set-output name=release_id::$RELEASEID"        env:          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}      # Publish the release draft      - name: Publish release        uses: eregon/publish-release@v1        env:          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}        with:          release_id: ${{ steps.get_release_id.outputs.release_id }}      # Get payload info from the pull request      - name: Payload info        id: payload_info        uses: Dovyski/payload-info-action@master        continue-on-error: true      # Remove the release or hotfix branch after publishing      - name: Remove PR branch        uses: dawidd6/action-delete-branch@v3        with:          github_token: ${{github.token}}          branches: ${{ steps.payload_info.outputs.branch }}
```

Merge main into develop
----------

After a release branch is merged into the main branch, it should be merged back into the "develop" branch. Optionally, you could set GitHub actions to create a commit that updates a CHANGELOG.md file before this merge occurs.

```
on:  push:    branches:      - mainpermissions:  contents: writejobs:  sync_develop:    runs-on: ubuntu-latest    steps:      - name: Checkout        uses: actions/checkout@master      - name: Merge main -> develop        uses: devmasx/merge-branch@master        with:          type: now          from_branch: main          target_branch: develop          github_token: ${{ secrets.GITHUB_TOKEN }}          message: Merged main into develop
```

Draft releases on "hotfix/\*" push
----------

When you must support a previous minor or major version, you should start by checking out that tag. Make sure to checkout the latest patch for that minor or major version. Then, you should create a "support/\*.x" branch. Support branches should function similarly to the "main" branch. Tags will be created when "hotfix/\*" branches are merged into it, and commits should **not** be added to it directly. After creating the support branch, a "hotfix/\*" branch with the next patch number should be created. These are similar to the "release/\*" branches, as they should be reviewed in a pull request, and when they are merged, Git actions should **publish** the drafted release and delete the "hotfix/\*" branch (similar to when a "release/\*" branch is merged, see the above example).

```
on:  push:    branches:      - hotfix/*jobs:  update_release_draft:    permissions:      contents: write    runs-on: ubuntu-latest    steps:      - name: Checkout        uses: actions/checkout@master      # Get the hotfix tag      - name: Get hotfix Tag        id: get_hotfix_tag        run: |          RELEASETAG=$(git branch | grep \* | sed -re "s/hotfix\///;s/\*//;s/\s*//g")          echo "::set-output name=release_tag::$RELEASETAG"      # Draft Release with hotfix branch      - name: Draft Release with hotfix branch        id: update_release_draft_with_hotfix_branch        uses: release-drafter/release-drafter@master        with:          tag: ${{ steps.get_hotfix_tag.outputs.release_tag }}        env:          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Merge support into main and develop
----------

If a bug existed in a previous version, it is likely that it still exists in the latest version too. Similar to "release/\*" branches that are merged into the main branch, support branches too should be merged into the main branch and then merged again into the develop branch when a "hotfix/\*" branch is merged into one. There is, however, a higher potential for this merge to conflict with the main branch. In that case, a developer must resolve the conflict on their own, and push the resolved merge into the main branch.

```
on:  push:    branches:      - support/*permissions:  contents: writejobs:  sync_develop:    runs-on: ubuntu-latest    steps:      - name: Checkout        uses: actions/checkout@master      # Get the support branch      - name: Get support branch        id: get_support_branch        run: |          SUPPORTBRANCH=$(git branch | grep \* | sed -re "s/[\*|\ ]//g")          echo "::set-output name=support_branch::$SUPPORTBRANCH"      - name: Merge support -> main        uses: devmasx/merge-branch@master        with:          type: now          from_branch: ${{ steps.get_support_branch.outputs.support_branch }}          target_branch: main          github_token: ${{ secrets.GITHUB_TOKEN }}          message: Merged ${{ steps.get_support_branch.outputs.support_branch }} into main      - name: Merge main -> develop        uses: devmasx/merge-branch@master        with:          type: now          from_branch: main          target_branch: develop          github_token: ${{ secrets.GITHUB_TOKEN }}          message: Merged main into develop
```

I hope you decide to use git-flow in your projects! It adds much-needed sanity to the management of multiple versions of software.

