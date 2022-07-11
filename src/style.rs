use std::borrow::            	Cow;
use futures_signals::signal::	Mutable;
use web_sys::                	{Element, HtmlElement, EventTarget};
use dominator::              	{DomBuilder, stylesheet, apply_methods, pseudo, clone, events, traits::MultiStr};
use once_cell::sync::        	Lazy;


pub fn init_stylesheet() {
  stylesheet!("html, body, #app", {
    .style("margin" 	, "0")
    .style("padding"	, "0")
  });
  stylesheet!("icon-btn", {
    .style("font-size"	, "0.9em")
  });
  // @apply inlines existing utility classes into a custom CSS, so the .class'es below have no effect unless removed from the main.css file and added to the popup/options.tsx elements
  // a parallel windicss JS build pipeline needed to add a css file with .btn styles
    // .btn { @apply px-4 py-1 rounded inline-block... hover:bg-teal-700 ... disabled:cursor...
    // .icon-btn {@apply inline-block ... opacity-75 ...
}

static CLASS_BUTTON_HOVER     :Lazy<[&str;1]> = Lazy::new(|| ["hover:bg-teal-700"]);
static CLASS_BUTTON_ICON_HOVER:Lazy<[&str;2]> = Lazy::new(|| ["hover:opacity-100","hover:text-teal-600"]);

pub fn mixin_btn     <A>(dom:DomBuilder<A>) -> DomBuilder<A>
  where A:AsRef<HtmlElement> + AsRef<Element> + AsRef<web_sys::EventTarget> {
  //            ↑ for .class        ↑ for .style
  apply_methods!(dom, {
    .class(["btn","mt-2"])
    .class(["px-4","py-1","rounded","inline-block"])
    .class(["bg-teal-600","text-white","cursor-pointer"])
    // .class(            *CLASS_BUTTON_HOVER)  // a. statically  set class, CSS :hover handles hover
    .apply(class_on_hover(*CLASS_BUTTON_HOVER)) // b. dynamically set class via a dom signal on hover
    .class(["disabled:cursor-default","disabled:bg-gray-600","disabled:opacity-50"])
  }) }
pub fn mixin_icon_btn<A>(dom:DomBuilder<A>) -> DomBuilder<A>
  where A:AsRef<    Element> + AsRef<web_sys::EventTarget> {
  apply_methods!(dom, {
    .class(["inline-block","cursor-pointer","select-none"])
    .class(["opacity-75","transition","duration-200","ease-in-out"])
    // .class(            *CLASS_BUTTON_ICON_HOVER)
    .apply(class_on_hover(*CLASS_BUTTON_ICON_HOVER))
  }) }



pub fn class_on_hover   <A,N>(name:N)                        -> impl FnOnce(DomBuilder<A>)
  -> DomBuilder<A>
  where N:MultiStr + 'static,      // MultiStr: literals and ["arrays"]
    //  N:Into<Cow<'static, str>>, // Cow: string literals + String, static + dynamic names
        A:AsRef<EventTarget> + AsRef<Element> { // can do c_on_h("foo") or c_on_h(format!(...))
  let is_hovering	= Mutable::new(false);
  // let name    	= name.into(); // needed with Cow

  move |dom| apply_methods!(dom, {.class_signal(name, is_hovering.signal())
    .event(clone!(is_hovering => move |_:events::MouseEnter| {is_hovering.set_neq(true );}))
    .event(                      move |_:events::MouseLeave| {is_hovering.set_neq(false);})
  }) // clone! helper to .clone values that are then moved into a closure (due to 'static requirements of many of the web apis move/clone are frequently required)
}
pub fn class_on_hover_ext<A,N>(name:N, is_hovering:&Mutable<bool>) -> impl FnOnce(DomBuilder<A>)
  // access hover state in my App: coh("a",↑&App.is_hover) (put Mutable<bool> into App's state)
  // to keep it private            coh("a",&Mutable::new(false))
  -> DomBuilder<A> + '_
  where N:MultiStr + 'static,
        A:AsRef<EventTarget> + AsRef<Element> {

  move |dom| apply_methods!(dom, {.class_signal(name, is_hovering.signal())
    .event(clone!(is_hovering => move |_:events::MouseEnter| {is_hovering.set_neq(true );}))
    .event(clone!(is_hovering => move |_:events::MouseLeave| {is_hovering.set_neq(false);}))
  })
}




use dominator::            	{class};
static CLASS_CSS_TEST      	: Lazy<String> = Lazy::new(|| class! {
  .style("background-color"	, "orange")
  .style("margin"          	, "5px") });
pub fn mixin_pseudo_style<A>         (dom:DomBuilder<A>) -> DomBuilder<A>
  where A:AsRef<Element> + std::convert::AsRef<web_sys::EventTarget> {

  let test:String = class! {
    .style("background-color", "green")
    .pseudo!(":hover"   , {.style("background-color","orange")})
    .pseudo!("::after"  , {.style("background-color","green" )})
    .pseudo!(":disabled", {.style("background-color","red"   )})
  };
  apply_methods!(dom, {
    .class(["class_tst3"])
    // .apply(class_on_hover(&*CLASS_CSS_TEST))
    // .apply(class_on_hover(CLASS_BUTTON_HOVER))
    .class(test)
  }) }

  // .apply(class_on_hover(CLASS_BUTTON_HOVER))
  // .apply(test)
  // .apply(.style("background-color", "black"))
