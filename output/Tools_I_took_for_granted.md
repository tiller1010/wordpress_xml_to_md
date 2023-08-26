created: 2020-03-08 17:30:08 +00:00
tags: [cms]

# Tools I took for granted


With today's modern work-flow, it is easy to feel spoiled with all of our handy tools. Git allows us to rollback to any previous commit in our application, just in case something were to go horribly wrong. A CMS allows us to easily update content without ever touching the code. Package managers like NPM and Composer can provide our project with all of its dependencies simply by running a command. But developers are not always working on new code. A majority of time is spent working on existing code. Some of that code may feel severely antiquated from what we are used to.

Imagine starting on a new project without Git. Scary right? Git is an essential part of today's development. However, you may find yourself working on an older project that does not use version control. To safely make changes you will need to create a backup of every file you change. That way if you do break something you are left SOL. When you are ready to make your changes live, you cannot just pull to a repository. You will need to FTP every file you changed individually. All of this is a huge headache by today's standards. Not only are you at a much higher risk of creating irreversible damage, but you must keep track of moving parts manually.

Content management systems make customizing your pages very simple. With a CMS you can upload content to your site without any need to touch the code. But what if you have a static site made up of several HTML files and no CMS? If you need to make a change to the frame of your site (navigation, footers, etc...), that change will have to be made across several files. Now, you could use a search and replace tool to make this happen all at once, but what if you have something like a "selected" class on navigation items? Each page will have the same navigation but with a unique selected value. You can start to imagine the headache of editing nearly identical elements across several files. With most content management systems, this is all taken care of.

With package managers, by running a single command, you can install all of your projects dependencies. Older projects may not be fortunate enough to include this. When working on a project like this, you may need to individually initiatialize and update submodules. It can be unclear what packages your project requires You may find yourself refreshing your browser until the errors are gone. With tools like Composer and NPM, these concerns are eliminated, so you can spend more time working on your project than resolving dependency issues.

Modern tools make our lives as developers much easier. However, we still find ourselves supporting older projects without these amenities. Working with these systems is far from impossible, but more caution and patience is often required.

