use dominator::   	{Dom, html, svg};
use crate::style::	*;

// TODO: a) import an svg file instead of the raw code
// TODO: b) any chance for an auto-import solution from the central database by name? or do you need a custom plugin to do that?

pub fn logo() -> Dom {
  html!("a", {
    .class(["icon-btn","mx-2","text-2xl"])
    .apply(mixin_icon_btn)
    .attr("rel"   	, "noreferrer")
    .attr("href"  	, "https://github.com/antfu/vitesse-webext")
    .attr("target"	, "_blank")
    .attr("title" 	, "GitHub")
    .child(icon_pixelarticons_power_raw())
  })
}

fn icon_pixelarticons_power_raw() -> Dom {
  svg!("svg", {
    .attr("preserveAspectRatio", "xMidYMid meet")
    .attr("viewBox", "0 0 24 24")
    .attr("width", "1.2em")
    .attr("height", "1.2em")
    .child(
      svg!("path", {
        .attr("fill", "currentColor")
        .attr("d", "M20 2h-2v4H6v2H4v8h2v2h2v4h8v-2h4v-2h-4v-2h4v-2h-4v-2H8v4H6V8h12V6h2V2zm-6 18h-4v-6h4v6z")
      })
    )})
}
