use leptos::*;

#[component]
pub fn Pago<'a>(pagado:bool, monto:ReadSignal<f32>,medios_pago:Vec<&'a str>,set_monto: WriteSignal<f32>) -> impl IntoView {
    let (input,button)=match pagado{
        true => (view!{<input type="number" prop:placeholder={move || monto.get()} disabled={true} class="input-monto" step="0.01"/>},
        view!{<button type="submit">"Borrar"</button>}),
        false => (view!{<input prop:value={move || monto.get()} class="input-monto" step="0.01"/>},view!{<button type="submit" on:click=move |ev| {ev.prevent_default();logging::log!("aca {}",monto.get());set_monto.update(|n|*n+=1.1);} >"Agregar"</button>}),
    };
    let mut medios=Vec::new();
    for medio in medios_pago{
        medios.push(view!{<option> {medio.to_string()}</option>})
    }
    let select=view!{
        <select class="opciones-pagos">
            {medios}
        </select>
    };

    view!{
        <form class= "pago">
        {input}
        {select}
        {button}
        </form>
    }
}