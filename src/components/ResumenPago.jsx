import { invoke } from "@tauri-apps/api/tauri";
import ItemResumen from "./ItemResumen";
import { useEffect } from "react";
import { useState } from "react";
import Pagos from "./Pagos";

function ResumenPago({ pos, venta, configs, prodFoc, isProd, credito,setDisabledCli }) {
    const [prods, setProds] = useState("");
    const [focus, setFocus] = useState(prodFoc);
    const [cred,setCred] = useState(credito);
    const [rend, setRend] = useState(<aside id="resumen-y-pago">
    <article id="resumen">
        {prods}
    </article>
    <Pagos prodFoc={focus} setDisabledCli={setDisabledCli} credito={cred} pagos={venta.pagos} medios_pago={configs.medios_pago} monto={venta.monto_total - venta.monto_pagado} pos={pos} isProd={isProd} />
</aside>)
    useEffect(()=>{
        setRend(<aside id="resumen-y-pago">
    <article id="resumen">
        {prods}
    </article>

    <Pagos prodFoc={focus} setDisabledCli={setDisabledCli} credito={cred} pagos={venta.pagos} medios_pago={configs.medios_pago} monto={venta.monto_total - venta.monto_pagado} pos={pos} isProd={isProd} />

</aside>)},[venta,focus,prods,configs,pos])
    useEffect(()=>{setCred(credito)},[credito]);
    useEffect(() => {setFocus(prodFoc)}, [prodFoc])

    useEffect(() => {
        async function get_descripcion_valuable(prod, conf) {
            return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
        }
        let resumenes = [];
        if (venta.productos.length > 0)
            venta.productos.forEach((prod, i) => {
                get_descripcion_valuable(prod, configs).then(desc => {
                    resumenes.push(<ItemResumen key={i} descripcion={desc} />)
                    if (venta.productos.length == resumenes.length) {
                        setProds(resumenes)
                    }
                })
            });
        else
            setProds([])
    }, [venta,pos,configs])


    return (rend)
}

export default ResumenPago;