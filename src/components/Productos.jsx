
import { useState } from "react";
import { useEffect } from "react";
import Producto from "./Producto";
import "./Productos.css"
function Productos({ handleProd,productos, conf, prodFoc,isProd}) {
    //const [state,setState] = useState()
    const [unfoc,setUnfoc]=useState(prodFoc?"":"not-focused");
    const [prods, setProds] = useState(productos.length > 0 ? productos.map((prod, i) => {
        return <Producto handleProd={handleProd} key={i} producto={prod} conf={conf} i={i} />
    }) : "")
    useEffect(()=>{
        setUnfoc(prodFoc?"":"not-focused")
    }, [prodFoc])
    useEffect(() => {
        setProds(productos.length > 0 ? productos.map((prod, i) => {
            return <Producto handleProd={handleProd} key={i} producto={prod} conf={conf} i={i} />
        }) : "")},[productos])
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