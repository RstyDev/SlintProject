pub fn input_style()->String{
  String::from("
    input{
      outline: none;
      border-radius: 8px;
      border: 1px solid transparent;
      padding: 0.6em 1.2em;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      color: #0f0f0f;
      background-color: #ffffff;
      transition: border-color 0.25s;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }
  ")
}
pub fn style()->String{
    String::from("
        :root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f9f9f9;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  -webkit-user-select: none;
  user-select: none;
}
input,
button {
  outline: none;
}
input:disabled {
  -webkit-user-select: none;
  user-select: none;
}
      input[type=number] {
  appearance: textfield;
  -moz-appearance: textfield;

  &::-webkit-outer-spin-button,
  &::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
}
input,
button,
select,
.a-boton {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}
form {
  display: grid;
  grid-template-columns: 40% 60%;

  & label {
    grid-column-start: 1;
  }

  & input {
    grid-column-start: 2;
  }

  &.pago {
    border-radius: 0.5px;
    grid-template-columns: 30% 50% 20%;

    & input {
      grid-column-start: 1;
      padding: 8px 10px;
    }

    & select {
      grid-column-start: 2;
      box-sizing: border-box;
      padding: 0 3px;
    }

    & input[type=submit],
    & input[type=button] {
      grid-column-start: 3;
      padding: 0 3px;
    }
  }
}

#menu-image button {
  display: inline-block;
  align-self: flex-start;
  width: 5%;
}

#menu-image button img {
  position: relative;
}

#menu-image {
  width: fit-content;
  height: fit-content;

}

#tabla-productos {
  height: fit-content;
}

#tabla-productos tr {
  max-height: 2em;
}

#tabla-productos tr:hover {
  background-color: #666;
}

.confirm-body {
  display: flex;
  flex-direction: column;
  align-items: center;

  & section {
    display: inline-block;

    & button {
      width: 5em;
    }
  }
}

.contenedor {
  display: block;
  position: relative;
  padding-left: 35px;
  margin-bottom: 12px;
  cursor: pointer;
  font-size: 22px;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;

  & input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }

  & .checkmark:after {
    left: 9px;
    top: 5px;
    width: 5px;
    height: 10px;
    border: solid white;
    border-width: 0 3px 3px 0;
    -webkit-transform: rotate(45deg);
    -ms-transform: rotate(45deg);
    transform: rotate(45deg);
  }
}

.checkmark {
  position: absolute;
  top: 8px;
  left: 0;
  height: 25px;
  width: 25px;
  background-color: #eee;

  &:after {
    content: '';
    position: absolute;
    display: none;
  }
}

.contenedor input:checked~.checkmark:after {
  display: block;
}

.contenedor:hover input~.checkmark {
  background-color: #ccc;
}

.contenedor input:checked~.checkmark {
  background-color: #2196F3;
}

.articulo {
  display: grid;
  grid-template-columns: auto 180px 80px 80px 80px;
  border: solid 1px #6f6f6f;
  border-radius: 10px;
  box-shadow: 3px 2px rgba(0, 0, 0, 0.2);
  padding-left: 10px;
}

#cuadro-principal .articulo:first-of-type {
  border-radius: 0px;

  & section,
  &.cantidad {
    border-right: solid 1px #6f6f6f;
    display: block;

  }

  & p:is(p) {
    text-align: center;
    margin: auto 0;
  }

  & .monto+section {
    grid-column-start: 4;
    grid-column-end: 6;
  }
}

.articulo p {
  margin: 5px 0;
  overflow-x: auto;
}

.descripcion {
  grid-column-start: 1;
  overflow-x: auto;
}



.cantidad button,
.cantidad p,
#borrar .button {
  margin: auto 5px;
}

.monto {
  grid-column-start: 3;
  text-align: end;
}

.monto+section {
  text-align: end;
}

.articulo section {
  padding: 5px;
}



#barra-de-opciones {
  width: auto;
  display: none;
  flex-direction: column;
  border-radius: 5px;
  background-color: #0000;
  position: absolute;
}

#barra-de-opciones.visible {
  display: inline-flex;
}

#header {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.container {
  margin: 0;
  padding: 1vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}



.boton {
  height: 2.6em;
  padding: 0.4em;
  will-change: filter;
  transition: 0.5s;
  background-color: #fafafa;
  border-radius: 8px;
  border: 1px;
  box-sizing: border-box;
}

@keyframes mov-error {
  0% {}

  25% {
    transform: translateY(3px);
  }

  50% {
    transform: translateY(0px);
  }

  75% {
    transform: translateY(-3px);
  }
}

.error {
  filter: drop-shadow(0px 0px 0px red);

  -webkit-animation: mov-error 0.2s infinite;
  -moz-animation: mov-error 0.2s infinite;
  -o-animation: mov-error 0.2s infinite;
  animation: mov-error 0.2s infinite;
}

.focuseado {
  border: solid 2px white;
  border-radius: 3px;

}





.v-actual {
  transform: scale(1.3);
}

#productos {
  gap: 5px;
  max-height: 93%;
  border-bottom: solid 1px #6f6f6f;
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;

  &::-webkit-scrollbar {
    display: none;
  }
}


#cuadro-principal section {
  display: flex;
  flex-direction: column;

  &#cuadro-venta {
    border: solid 1px #6f6f6f;
    width: 99%;
    height: 78svh;

    & .focuseable {
      border: solid 1px #535bf2;

      &.not-focused {
        border-color: #6F6F6F;
        filter: blur(2px);
        transform: scale(0.99);
      }
    }
  }
}


.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

table,
th,
td {
  border-collapse: collapse;
}

tr:nth-child(even) {
  background-color: #888;
}

tr:hover {
  border: 1px solid #bbbbbb;
}


a:hover {
  color: #535bf2;
}

#resumen {
  height: 70%;
  font-size: 0.75em;
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;

  &::-webkit-scrollbar {
    display: none;
  }
}

div#pagos section.pago {
  display: flex;
  flex-direction: row;
}

#resumen-y-pago {
  padding-right: 10px;
  height: 77.5svh;
  width: 350px;
  & p:is(p) {
    align-self: self-end;
    margin: 2px;
  }
}



#pagos {
  height: 30%;
  display: flex;
  border: solid 1px #6f6f6f;
  padding: 3px;
  flex-direction: column;
  overflow-y: scroll;
  -ms-overflow-style: none;
  scrollbar-width: none;
  margin-left: 5px;

  &::-webkit-scrollbar {
    display: none;
  }

  &.focuseable {
    border-color: #535bf2;

    &.not-focused {
      border-color: #6F6F6F;
      filter: blur(2px);
      transform: scale(0.99);
    }
  }
}

/* .input-monto {
  width: 20%;

  &::-webkit-outer-spin-button,
  &::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
} */






h1 {
  text-align: center;
}

header button {
  display: contents;
}

#monto-total {
  display: inline-block;
  font-size: 1.5em;
  font-weight: 600;
  align-self: end;
  margin: auto 5px 5px 5px;
}

#monto-total p {
  display: inline-block;
  padding: 5px 10px;
  background-color: #fafafa;
  border-radius: 5px;
}

.descripcion {
  flex-grow: 1;
}




select,
.a-boton {
  background-color: #fafafa;
  color: #9a9a9a;
}

.a-boton {
  height: 20px;
}


.eliminar {
  padding: 0.6em 0.5em;
  width: 70px;
  font-size: 90%;
}

main {
  display: flex;
  justify-content: center;
}





label {
  display: block;
  padding: 10px;
}

#input-nombre-proveedor {
  display: block;
  align-self: center;
  margin: auto;
}
      input,
  button,
  select {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

.Lower {
  text-transform: lowercase;
}

.Upper {
  text-transform: uppercase;
}

.Camel {
  text-transform: capitalize;
}

#agregar-proveedor-submit div {
  text-align: center;
  width: 100%;
}

.add-form {
  display: flex;
  flex-direction: column;
}

#form-login {
  height: 90svh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  & * {
    margin: 5px;
  }
}

#agregar-producto-container {
  display: grid;
  grid-template-columns: 35% 65%;
  grid-template-rows: auto auto;

  & #agregar-producto {
    grid-column-start: 2;
    grid-row: 1/span 2;
  }

  & #input-codigo {
    margin-top: 20px;
    grid-column-start: 1;
  }

  & #agregar-proveedor-producto {
    grid-column-start: 1;
  }

  & form {
    display: flex;
    flex-direction: column;
    align-items: center;

    & section {
      display: inline-block;
    }
  }

}

#cerrar-agregar-proveedor,
#cerrar-agregar-producto,
#cerrar-cambiar-configs {
  align-self: flex-end;
}



button:hover,
.boton:hover,
.a-boton:hover {
  border-color: #396cd8;
  border-style: solid;
  cursor: pointer;
  color: #7f7f7f;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

*:disabled {
  color: #777;
  background-color: #aaa;
  transition: none;
}

*:disabled:hover {
  border: #aaa;
  cursor: auto;
  border-color: rgba(0, 0, 0, 0);
  border-radius: 8px;
  border-style: solid;
  border-width: 1px;
  border-image-outset: 0;
}



#mensaje1-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }





  button:active {
    background-color: #0f0f0f69;
  }



  select,
  .a-boton {
    background-color: #1a1a1a;
    color: #7f7f7f;
  }

  #monto-total p {
    background-color: #2f2f2f;
  }

  .boton {
    height: 2.6em;
    padding: 0.4em;
    will-change: filter;
    transition: 0.5s;
    background-color: #1a1a1a;
    border-radius: 8px;
    border: 1px;
    box-sizing: border-box;
  }

  *:disabled {
    color: #666666;
    background-color: #2f2f2f;
  }

  .main-screen {
    border: solid 2px #8f8f8f;
    background-color: #0f0f0f50;
  }


  *:disabled:hover {
    background-color: #2f2f2f;
  }

  *:disabled:hover {
    border: none;
  }
}")
}