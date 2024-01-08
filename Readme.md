# Hrtml a templating language that aims to be as close as possible to html.
Hrtml is a Rust Templating Markup Language. (HRTML)

Is it a recursive acronym? Yes.

## Project in early stages.
This is a very new project.

RoadMap:
- [ ] Finalize Syntax.
- [ ] Lexer.
- [ ] Parser.
    - [ ] Attribute Parsing.
    - [ ] Template Parameters Aggregation.
    - [ ] Template MetaData Aggregation.
        - [ ] Parameters.
            - [ ] Slices.
            - [ ] Toggles.
            - [ ] Slots.
            - [ ] Match.
            - [ ] Each.
        - [ ] Imports
- [ ] CodeGen.

## How you interact with templates.
Templates are compiled to rust code. There is two way to compile templates:
- Use the `build.rs` to build multiple templates into modules that you include using the `include` macro.
- Use the macros provided by the crate to include templates files one by one.

Both approaches have advantages an drawbacks. The first tends to break easily lsps but is less prone to module missplacement errors.
The second is easier but tends to be more verbose if you have a lot of templates.

The methods are gated behind features.

## Template syntax.

HRTML aims to be as close as possible to classic html template syntax. It makes use of a few special tags to make the templating work. It also alows you to add classes and ids more easily with a syntax inspired by maud.

### Importing other templates.
To import other templates use the `import` void (self closing) tag with the `from` attribute.

```html
<!-- In Index.hrtml -->
<import from=`Hello`/>

<Hello> World! </Hello>

<!-- In Hello.hrtml -->
<p> Hello <slot/> </p>

<!-- Generates the folowing html. -->
<p> Hello World! </p>
```

### Slots.
Just like elements template can have children. To controll where children go in a template we use the `slot` tag.
```html
<!-- In Hello.hrtml -->
<p> Hello <slot/> </p>
```
Any chidren of the `Hello` template element will be placed inside the slot.

But what if i want to put multiple child elements in different places in the template. Worry not we introduce identified slots.
To create a named slot simply add the `id` attribute to your slot.
For example a two column template:

```html
<!-- In TwoCols.hrtml -->
<div .left>
    <slod id="left"/>
</div>
<div .right>
    <slod id="right"/>
</div>

<!-- In Index.hrtml -->
<import from=`TwoCols`/>
<TwoCols>
    <p slot-id="left">
        I'm first on the left.
    </p>
    <p slot-id="right">
        I'm first on the right.
    </p>
    <p slot-id="left">
        I'm second on the left.
    </p>
</TwoCols>

<!-- Expands to: -->
<div .left>
    <p>
        I'm first on the left.
    </p>
    <p>
        I'm second on the left.
    </p>
</div>
<div .right>
    <p>
        I'm first on the right.
    </p>
</div>
```

### Template Parameters.
Template parameters allows you to define types to control and render your templates. For a type to be a valid template parameter it must either:
- be an implementor of the `Render` trait or an `Iterator` of `Render` implementors.
- be a value of `RawHtml` (output of Render::render).
- be an implementor of the `Attribute` trait or an `Iterator` of `Atribute` implementors.
- be an implementor of the `AttributeValue` trait or an `Iterator` of `Atribute` implementors.
- be an implementor of the `Truthy` trait.

#### Render Parameters.
For a parameter to be rendered into the template the template must contains `slot` elments with the attribute `name`. The struct fields must correspond with the slot names. You can alternatively add a `#[rename(new_name)]` attribute on a struct field to assign it to a slot. If the template is imported in another template you can use the `slot-<name>=new_name` attribute on the template element to pass down a template attribute to the imported template.

#### Toggles and Slices.
Sometimes we want to add classes, ids or attributes conditionaly. It can be done with toggles. The syntax for toggles is as folows `[template_param]thingtotoggle`. The template parameters in toggles must implement `Truthy`.
```html
<p
    [toggle_italic].italic-txt
    [toggle_id]#some-text
    [toggle_red]style="color: red;"
>
    This is text.
</p>
```
To enable a truthy parameter simply add it to the template element.
```html
<!-- Text.hrtml -->
<p
    [red]style="color: red;"
    [green]style="color: green;"
>
    This is text.
</p>
<!-- Index.hrtml -->
<import from="Text">
<Text red/>
<Text green/>
<!-- Expands to: -->
<p
    style="color: red;"
>
    This is text.
</p>
<p
    style="color: green;"
>
    This is text.
</p>
```

Sometimes we want to add classes, ids or attributes dynamicly. It can be done with splices. The params with splices must implement the `Atribute` trait. The syntax for splices is as folows `(template_param)`.

If a type implment `Truthy` and `Attribute` it can be used in a toggleable slice as folows `[(template_param)]`.
For example `Option<T>` is `Truthy` if `T` implements `Attribute`.

### Control Flow.
To render html conditionaly we use the `if` tag with the attribute `cond` wich accepts a value from a `Truthy` parameter. 
```html
<!-- Enable.hrtml -->
<if cond=enable>
    <p>I'm enabled.</p>
</if>
<elseif cond=>
    <p>I'
</elseif>
<else>
    <p>I' disabled</p>
</else>
<!-- Index.hrtml -->
<import from="Enable">

<Enable enable/>
<Enable/>

<!-- Expands to: -->
<p>I'm enabled.</p>
<p>I' disabled</p>
```

The `match` 
```html
<match val=value>
    <case pat="">

    </case>
    <case pat="">

    </case>
</match>

```

### Loops. 
The `each` element allows you to render a parameter implementing `Iterator` wich items are `Render`. Effectively rendering reapeating patterns like cards.

```html
<!-- Cards.hrtml -->
<each elems=cards>
    <div>
        <h1><slot name=title/></h1>
        <p><slot name=date/></p>
        <img src=(img_src) />
        <p><slot name=summary/></p>
    </div>
</each>
<!-- Expands to: -->
<div>
    <h1>My last article.</h1>
    <p>today</p>
    <img src="data/imgs/lastarticle.png"/>
    <p>The last article i've writen.</p>
</div>
<div>
    <h1>An oler article.</h1>
    <p>3days ago</p>
    <img src="data/imgs/oldarticle.png"/>
    <p>I don't remember what i said.</p>
</div>
```

Asumming the template is called with a parameter of type `impl Iterator<Item = cards::Params>` and 
```rust
mod cards::Params {
    title: impl Render,
    date: impl Render,
    img_src: impl AttributeValue,
    summary: impl Render,
}
```

