use leptos::*;

#[component]
pub fn Pago<'a>(pagado:bool, monto:f32,medios_pago:Vec<&'a str>) -> impl IntoView {
    let input=match pagado{
        true => view!{<input type="number" placeholder={monto} disabled={true} class="input-monto" step="0.01"/>},
        false => view!{<input value={monto} class="input-monto" step="0.01"/>},
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
        </form>
    }
}