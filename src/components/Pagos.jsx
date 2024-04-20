import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";
import Pago from "./Pago";



async function agregar_pago(medio_pago, monto, pos) {
  return await invoke("agregar_pago", { "medioPago": medio_pago, "monto": monto, "pos": pos });
}

function Pagos({ pagos, medios_pago, monto, pos, isProd, prodFoc }) {
  const [pagosVec, setPagosVec] = useState(mapearPagos(pagos))
  const [focused, setFocused] = useState(prodFoc?"not-focused":"");
  useEffect(() => {
    setFocused(prodFoc?"not-focused":"")
  }, [prodFoc])




  return (<>
    <article id="pagos" className={"focuseable " + focused} onClick={() => { isProd(false)}} >
      {pagosVec}
      <Pago pagado={false} medios_pago={medios_pago} monto={monto} pos={pos} borrar={(e) => { console.log(e); borrar_pago(pos, e) }} agregar={cash} />
    </article>
    <p>Resta pagar: {monto}</p>
  </>
  )
 
  function mapearPagos(pagos) {
    return pagos.map(function (pago, i) {
      return <Pago key={i} pagado={true} medios_pago={[pago.medio_pago.medio]} monto={pago.monto} index={i} borrar={(e) => borrar_pago(pos, e)} agregar={cash} />
    })
  }
  function cash(e, seleccionado, montoAct) {
    e.preventDefault();
    console.log("cash")
    agregar_pago(seleccionado, montoAct, pos).then(pagos => {console.log(pagos);setPagosVec(mapearPagos(pagos))});
  }
}


export default Pagos;