use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center font-semibold space-y-1">
            <div class="text-xl">
                "Dancing with My "
                <span class="text-mycolor-1">"C"</span>
                <span class="text-mycolor-2">"o"</span>
                <span class="text-mycolor-3">"d"</span>
                <span class="text-mycolor-4">"e"</span>
            </div>
            <div class="text-base font-light">"ダンシング・ウィズ・マイ・コード"</div>
        </div>
    }
}
