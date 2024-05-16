import React from "react";
import ReactDOM from "react-dom/client"
import Form from "./Form";
import "./../../styles.css"


ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <Form message={"Desea cancelar la venta?"} />
    </React.StrictMode>
);