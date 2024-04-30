import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import "./Producto.css"

async function get_descripcion_valuable(prod, conf) {
    return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
}

const procesarPes = async (cantidad,setCantidad,handle, prod, conf, i, setRet) => {
    let disabled = prod.Pes[0] <= 1 ? "disabled" : "";
    
    get_descripcion_valuable(prod, conf).then(desc => {
        setRet(
            <article id={i} className="articulo">
                <section className={"descripcion " + conf.modo_mayus}>
                    <p>{desc}</p>
                </section>
                <section className="cantidad">
                    <button className="button restar" disabled={disabled} onClick={()=>{setCantidad(parseFloat(cantidad-1));handle(i, -1)}}>-</button>
                    <input type="text" className="cantidad-producto" value={cantidad} onChange={(e)=>{setCantidad(parseFloat(e.currentTarget.value));}} onKeyDown={(e) =>{if (e.keyCode==13) handle(i, cantidad,true)}}/>
                    <button className="button sumar" onClick={()=>{setCantidad(parseFloat(cantidad)+1);handle(i, 1)}}>+</button>
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
                    <button className="button restar" disabled={disabled} onClick={()=>{setCantidad(parseInt(cantidad-1));handle(i, -1)}}>-</button>
                    <input type="text" className="cantidad-producto" value={cantidad} onChange={(e) => { setCantidad(parseInt(e.currentTarget.value));}} onKeyDown={(e) =>{if (e.keyCode==13) handle(i, cantidad,true)}}/>
                    <button className="button sumar" onClick={()=>{setCantidad(parseInt(cantidad)+1);handle(i, 1)}}>+</button>
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
const procesarProd = async (cantidad,setCantidad,handle, prod, conf, i, setRet) => {
    let disabled = prod.Prod[0] <= 1 ? "disabled" : "";
    let desc = await get_descripcion_valuable(prod, conf);
    
    setRet(<article id={i} className="articulo">
        <section className={"descripcion " + conf.modo_mayus}>
            <p>{desc}</p>
        </section>
        <section className="cantidad">
            <button className="button restar" disabled={disabled} onClick={()=>{setCantidad(parseInt(cantidad-1));handle(i, -1)}}>-</button>
            <input type="text" className="cantidad-producto" value={cantidad} onChange={(e) => { setCantidad(e.currentTarget.value);}} onKeyDown={(e) =>{if (e.keyCode==13) handle(i, cantidad,true)}} />
            <button className="button sumar" onClick={()=>{setCantidad(parseInt(cantidad)+1);handle(i, 1)}}>+</button>
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
    const [cantidad,setCantidad] = useState(()=>{
    if (Object.keys(producto)[0]=='Prod')
        return producto.Prod[0]
    else if(Object.keys(producto)[0]=='Pes')
        return producto.Pes[0]
    else if (Object.keys(producto)[0]=='Rub')
        return producto.Rub[0]
    });
    
    function procesar(){
        console.log(cantidad)
        switch (Object.keys(producto)[0]){
            case "Pes":
                procesarPes(cantidad,setCantidad,handleProd,producto, conf, i, setRet);
                break;
            case "Prod":
                procesarProd(cantidad,setCantidad,handleProd,producto, conf, i, setRet);
                break;
            case "Rub":
                procesarRub(cantidad,setCantidad,handleProd, producto, conf, i, setRet);
                break;
            default:
                console.error("Error de tipo de producto");
                break;
            }
        }
    useEffect(procesar,[producto,cantidad])
    return ret
}
                
export default Producto;