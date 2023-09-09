created: 2021-07-05T00:15:38+00:00
tags: [module]

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

