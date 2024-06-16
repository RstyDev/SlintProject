use leptos::*;

#[component]
pub fn Pago<'a>(pagado:bool, monto:ReadSignal<f32>,medios_pago:Vec<&'a str>,set_monto: WriteSignal<f32>) -> impl IntoView {
    let (input,button)=match pagado{
        true => (view!{<input type="number" prop:placeholder={move || monto.get()} disabled={true} class="input-monto" step="0.01"/>},
                 view!{<input id="boton-borrar-pago" type="button" prop:value="Borrar"/>}),
        false => (view!{<input prop:value={move || monto.get()} class="input-monto" step="0.01"/>},
                  view!{<input id="boton-agregar-pago" type="submit" on:click=move |ev| {ev.prevent_default();logging::log!("aca {}",monto.get());set_monto.update(|n|*n+=1.1);} prop:value="Cash"/>}),
    };
    let medios=medios_pago.iter().map(|med|view!{<option> {med.to_string()}</option>}).collect::<Vec<_>>();

    view!{
        <form class= "pago">
        {input}
        <select class="opciones-pagos">
            {medios}
        </select>
        {button}
        </form>
    }
}