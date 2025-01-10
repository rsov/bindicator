use rand::Rng;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Debug, PartialEq)]
pub struct CarouselProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Carousel(props: &CarouselProps) -> Html {
    let id_rand: String = format!("carousel_{}", rand::thread_rng().gen_range(100..1000));
    let id_target = format!("#{}", id_rand);

    html! {
    <div id={id_rand} class="carousel slide h-100">
      <div class="carousel-inner">
        {props.children.clone()}
      </div>
      <button class="carousel-control-prev" type="button" data-bs-target={id_target.clone()} data-bs-slide="prev">
        // <span class="carousel-control-prev-icon" aria-hidden="true"></span>
        <span class="visually-hidden">{"Previous"}</span>
      </button>
      <button class="carousel-control-next" type="button" data-bs-target={id_target} data-bs-slide="next">
        // <span class="carousel-control-next-icon" aria-hidden="true"></span>
        <span class="visually-hidden">{"Next"}</span>
      </button>
    </div>
        }
}

#[derive(Properties, Debug, PartialEq)]
pub struct CarouselItemProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub active: bool,
}

#[function_component]
pub fn CarouselItem(props: &CarouselItemProps) -> Html {
    let active_class = match props.active {
        true => "active",
        _ => "", // he-he-he-he
    };

    let item_class = format!("carousel-item {}", active_class);

    html! {
      <div class={item_class}>
          {props.children.clone()}
      </div>
    }
}
