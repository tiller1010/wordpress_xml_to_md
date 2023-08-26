created: 2021-01-26 02:33:05 +00:00
tags: [expo]

# Take Your Web App from React To React Native


Most of us Web Developers have never had the experience of building a mobile app. The task may seem daunting and out of the usual comfort zone. You might be starting with no Java or Swift experience. Learning both will take some time, and you would need to manage two codebases. Lucky for us, there is React Native and Expo.

<img alt="" src="https://tylertroutblog.com/wp-content/uploads/2021/01/reactNative-1024x512.png" height="228" width="456" />Create cross-platform mobile apps with React Native

React Native lets us developers write mobile apps in React, no Java or Swift experience required. Expo further simplifies the development process. It provides commonly used libraries. It sets up a development environment. It provides debugging tools and much more. With both of these tools, some React experience, and some wit, any web developer can develop their app for iPhone and Android. If you follow these steps, you too can transform your web-based React code into working React Native code.

<img alt="" src="https://tylertroutblog.com/wp-content/uploads/2021/01/expo.png" height="201" width="411" />Simplify development. Spend less time getting set up and more time building your app.

The "\<div\>" tag is perhaps the most commonly used element when it comes to web development. React Native has its own version of "\<div\>": the "\<View\>". Views are used to wrap content areas in your app's layout. Go ahead, copy and paste your web-based React code into your expo app, run a search for "div" and replace it with "View". Your project may contain other wrappers such as "\<section\>" or "\<article\>". Be sure to replace these as well. Your content wrappers are now native friendly, and you are one step closer to transitioning your app to mobile. Now, let's take care of the text content.

On the web, there are many different types of text tags. You could have headers in the form of "\<h1\>" down to "\<h6\>". Your project might have text in "\<span\>", "\<p\>", or "\<label\>" tags. Anywhere there is text, we will replace the tags with the "\<Text\>" element. Do this with any plain text in your project, but you can skip any links that use the "\<a\>" tag for now. This will be handled by a different React Native component, which we will cover next.

Links on the web use the anchor tag: "\<a\>" with an "href" attribute describing the link's destination. We will replace these links with the React Native "\<Button\>" element. The text within the link should be moved to the "title" prop of the Button (e.g. \<Button title="Click me"/\>). As for the "href", you will instead define a function to navigate to a screen. This will be placed within the "onPress" prop. [Here is a more in-depth guide](https://reactnative.dev/docs/navigation) on how to do this. Some links may also pass data in the form of "GET" variables (e.g. "http://example.com?foo=bar"). The variable "foo" has the value of "bar". In React Native, you can pass data as an object in the second argument of the navigation function. You would instead write something like: "navigation.navigate('Example', {foo: 'bar'})".

 You still may need to update "\<input\>"s to "\<TextInput\>"s or "\<video\>"s to "\<Video\>"s, but by now you should see that for every web element, there is a React Native equivalent component. Take some time to confirm each HTML tag is replaced with the appropriate component. With all of these changes, your app should be well on its way to working on iPhone and Android.

