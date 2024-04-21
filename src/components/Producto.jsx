import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import "./Producto.css"
async function get_descripcion_valuable(prod, conf) {
    return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
}
var ret;
const procesarPes = async (handle,prod, conf, i, setRet) => {
    let disabled = prod.Pes[0] <= 1 ? "disabled" : "";
    get_descripcion_valuable(prod, conf).then(desc => {
        setRet(
            <article id={i} className="articulo">
                <section className={"descripcion " + conf.modo_mayus}>
                    <p>{desc}</p>
                </section>
                <section className="cantidad">
                    <button className="button restar" disabled={disabled} onClick={()=>{handle(i, -1)}}>-</button>
                    <p className="cantidad-producto">{prod.Pes[0]}</p>
                    <button className="button sumar" onClick={()=>{handle(i, 1)}}>+</button>
                </section>
                <section className="monto">
                    <p></p>
                </section>
                <section></section>
                <section id="borrar">
                    <button className="button eliminar" onClick={()=>{handle(i, 0)}}>Borrar</button>
                </section>
            </article>)
    })
}
const procesarRub = async (handle,prod, conf, i, setRet) => {
    let disabled = prod.Rub[0] <= 1 ? "disabled" : "";
    get_descripcion_valuable(prod, conf).then(desc => {
        setRet(
            <article id={i} className="articulo">
                <section className={"descripcion " + conf.modo_mayus}>
                    <p>{desc}</p>
                </section>
                <section className="cantidad">
                    <button className="button restar" disabled={disabled} onClick={()=>{handle(i,-1)}}>-</button>
                    <p className="cantidad-producto">{prod.Rub[0]}</p>
                    <button className="button sumar" onClick={()=>{handle(i,1)}}>+</button>
                </section>
                <section className="monto">
                    <p></p>
                </section>
                <section></section>
                <section id="borrar">
                    <button className="button eliminar" onClick={()=>{handle(i,0)}}>Borrar</button>
                </section>
            </article>)
    })
}
const procesarProd = async (handle,prod, conf, i, setRet) => {
    let disabled = prod.Prod[0] <= 1 ? "disabled" : "";
    let desc = await get_descripcion_valuable(prod, conf);
    setRet(<article id={i} className="articulo">
        <section className={"descripcion " + conf.modo_mayus}>
            <p>{desc}</p>
        </section>
        <section className="cantidad">
            <button className="button restar" disabled={disabled} onClick={()=>{handle(i, -1)}}>-</button>
            <p className="cantidad-producto">{prod.Prod[0]}</p>
            <button className="button sumar" onClick={()=>{handle(i, 1)}}>+</button>
        </section>
        <section className="monto">
            <p>{prod.Prod[1].precio_de_venta}</p>
        </section>
        <section>
            <p>{prod.Prod[1].precio_de_venta*prod.Prod[0]}</p>
        </section>
        <section id="borrar">
            <button className="button eliminar" onClick={()=>{handle(i, 0)}}>Borrar</button>
        </section>
    </article>);
}

function Producto({ handleProd,producto, conf, i }) {
    const [ret, setRet] = useState("");
    useEffect(procesar,[producto])
    function procesar(){
        switch (Object.keys(producto)[0]){
            case "Pes":
                procesarPes(handleProd,producto, conf, i, setRet);
                break;
            case "Prod":
                procesarProd(handleProd,producto, conf, i, setRet);
                break;
            case "Rub":
                procesarRub(handleProd,producto, conf, i, setRet);
                break;
            default:
                console.error("Error de tipo de producto");
                break;
        }
    }
    return ret
}

export default Producto;