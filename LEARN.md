## Part 1: The DOM

- The [DOM](https://dom.spec.whatwg.org) is a tree of nodes. A node has zero or more children. ( It also has various other attributes and methods, but we can ignore most of those for now )

- There are several [Node Types](https://dom.spec.whatwg.org/#dom-node-nodetype), but for now we will ignore most of them and say that a node is either an Element or a Text node. In a language with inheritance these would be subtypes of Node. In Rust they can be an enum (Rust's keyword for a "tagged union" or "sum type"):

- An element includes a tag name and any number of attributes, which can be stored as a map from names to values. Robinson doesn't support namespaces, so it just stores tag and attribute names as simple strings.

## Part 2: HTML
- HTML has its own unique [parsing algorithm](https://html.spec.whatwg.org/multipage/syntax.html#parsing). Unlike parsers for most programming languages and file formats, the HTML parsing algorithm does not reject invalid input. Instead it includes specific error-handling instructions, so web browsers can agree on how to display every web page, even ones that don't conform to the syntax rules. Web browsers have to do this to be usable: Since non-conforming HTML has been supported since the early days of the web, it is now used in a huge portion of existing web pages.

- My parser can handle simple pages like this:
``` html
<html>
    <body>
        <h1>Title</h1>
        <div id="main" class="test">
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>
```
- The following syntax is allowed:
```
    . Balanced tags: <p>...</p>
    . Attributes with quoted values: id="main"
    . Text nodes: <em>world</em>
```