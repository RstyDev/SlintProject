use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};

#[component]
pub fn Pago<'a>(options:&'a ReadSignal<Vec<String>>) -> impl IntoView {
    let res=options.get().iter().map(|opt|{
        view!{<option>{opt}</option>}
    }).collect_view();
    view!{
        <input type="text" placeholder="Monto"/>
        <select>
            {res}
        </select>
        <button type="submit" >"Agregar"</button>
    }
}