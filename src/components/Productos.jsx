
import { useState } from "react";
import { useEffect } from "react";
import Producto from "./Producto";
import "./Productos.css"
function Productos({ productos, conf, prodFoc,isProd}) {
    const [unfoc,setUnfoc]=useState(prodFoc?"":"not-focused");
    useEffect(()=>{
        setUnfoc(prodFoc?"":"not-focused")
    }, [prodFoc])
    let prods=productos.length>0? productos.map((prod,i)=>{
        return <Producto key={i} producto={prod} conf={conf} i={i}/>
    }):"";
    return (<section id="productos" className={"focuseable "+unfoc} onClick={()=>isProd(true)}>
        <article className="articulo">
        <section className="descripcion">
            <p>DESCRIPCION</p>
        </section>
        <section className="cantidad">
            <p>CANTIDAD</p>
            </section>
        <section className="monto">
            <p>UNIDAD</p>
        </section>
        <section>
            <p>TOTAL PARCIAL</p>
        </section>
    </article>
    {prods}
    </section>)
}

export default Productos;