# spargel

> THE blog engine nobody asked for!

![design/logo_small.png](design/logo_small.png)

## Features

- Post display
- Pagination
- Feed (RSS)
- Simple categories
- Robots.txt generation on the fly (in case you need to hide some posts from Crawlers)
- Markdown support
- KI based* image preview
- File attachment upload and deletion

---
*or maybe just "if"..who knows

## Requirements

- Build with `rustc 1.77.0-nightly` or later.
- Build with `cargo build`

## Setup 

Make sure the folders `posts`, `cache` (both writable) and `static` are present besides the spargel binary.

```
{
    "title": "My title",
    "sub_title": "A good subtitle",
    "meta": {
        "SomeMetaKey": "ThisIsOptional"
    },
    "url": "https://yoururlwithouttrailingslash.example.tld",
    "theme": "terminal.css"
}

```

## Post Syntax

> Everything is a post. Also pages. Except when not.


**The HTML comments are only added for documentary purposes**

```
Title (will be sluggified)
Date (Y-m-d H:M)
:post,:hide,:anycategory,:commaseparatedbutprefixed
Your text, can be **Markdown**.
<!-- Add a newline at the end -->
```

## Is this production ready?

Of course not!

## License

AGPLv3