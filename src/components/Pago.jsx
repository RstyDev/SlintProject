import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";


function Pago({ pagado, medios_pago, monto, id,  borrar, agregar,credito,isProd }) {
    const [cred,setCred] = useState(credito);
    useEffect(()=>{setCred(credito)},[credito])
    const boton = pagado ? <input value="Borrar" onClick={(e)=>{isProd(false);borrar(e)}} type="button" id="boton-borrar-pago"></input> : <input value="Cash" onClick={(e)=>{isProd(false);agregar(e,seleccionado,montoAct)}} type="submit" id="boton-agregar-pago"></input>
    const [seleccionado, setSeleccionado] = useState(cred?medios_pago[0]:medios_pago[1]);
    const [montoAct, setMontoAct] = useState(""+monto);
    useEffect(()=>{setMontoAct(""+monto)},[monto])    
    const input = pagado ? <input type="number" onClick={()=>{isProd(false)}} placeholder={montoAct} readOnly={pagado} disabled="disabled" className="input-monto"  step="0.01" /> : 
        <input type="number" onClick={() => { isProd(false) }} value={montoAct}  onChange={(e)=>{
        setMontoAct(e.currentTarget.value)}} className="input-monto" id="input-activo" step="0.01" />
    
    const opts = medios_pago.map(function (opt, i) {
        let sel;
        if ((i == 0 && cred)||(i==1 &&!cred)||(pagado && i == 0)) {
            sel = "selected";

        }else{
            sel = ""
        }
        if(cred || i>0 || pagado)
        return <option key={i} id={i} defaultValue={sel} value={opt}>{opt}</option>
    });
    
    return (<form className="pago" id={id} onSubmit={(e) => agregar(e, seleccionado, montoAct)}>
        {input}
        <select  onChange={(e) => { setSeleccionado(e.currentTarget.value) }} className="opciones-pagos">
            {opts}
        </select>
        {boton}
    </form>)
}

export default Pago;