use yew::{function_component, html, AttrValue, Html, Properties};

#[function_component]
pub fn Bin() -> Html {
    html! {
        <div>
            {"This weeks bins are:"}
            <BinSVG color="green" />
            <BinSVG color="red" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BinSVGProps {
    pub color: AttrValue,
}

#[function_component]
fn BinSVG(&BinSVGProps { ref color }: &BinSVGProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="white" height="80px" width="80px" version="1.1" viewBox="0 0 297 297">
            <g>
                <path style={format!("fill:{}", color)} d="M221.74,94.57L207.98,259.6c-0.79,9.51-8.88,16.95-18.42,16.95h-82.12    c-9.53,0-17.63-7.44-18.42-16.95L75.27,94.57H221.74z"/>
                <path style={format!("fill:{}", color)} d="M239.49,65.69v8.43H57.51v-8.43c0-6.32,5.14-11.47,11.47-11.47h159.05    C234.35,54.22,239.49,59.37,239.49,65.69z"/>
                <path d="M259.94,65.69v18.66c0,5.64-4.58,10.22-10.22,10.22h-7.46l-13.9,166.73c-1.67,20.02-18.71,35.7-38.8,35.7h-82.12    c-20.08,0-37.13-15.68-38.8-35.7L54.75,94.57h-7.46c-5.65,0-10.23-4.58-10.23-10.22V65.69c0-17.6,14.32-31.91,31.92-31.91h35.56    v-4.73C104.54,13.03,117.57,0,133.59,0h29.82c16.02,0,29.06,13.03,29.06,29.05v4.73h35.56C245.63,33.78,259.94,48.09,259.94,65.69    z M239.49,74.12v-8.43c0-6.32-5.14-11.47-11.46-11.47H68.98c-6.33,0-11.47,5.15-11.47,11.47v8.43H239.49z M207.98,259.6    l13.76-165.03H75.27L89.02,259.6c0.79,9.51,8.89,16.95,18.42,16.95h82.12C199.1,276.55,207.19,269.11,207.98,259.6z M172.02,33.78    v-4.73c0-4.74-3.86-8.6-8.61-8.6h-29.82c-4.74,0-8.6,3.86-8.6,8.6v4.73H172.02z"/>
            </g>
        </svg>
    }
}