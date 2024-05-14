var contents;

function readSingleFile(e) {
    var file = e.target.files[0];
    if (!file) {
        return;
    }

    var reader = new FileReader();
    reader.onload = function (e) {
        var contents = e.target.result;
        let res = JSON.parse(contents);
        console.log(res[0]);
        let prods = []
        // for (let i = 0; i < res.length; i++) {
        //     let esta = false;
        //     for (let j = 0; j < prods.length; j++)
        //         if (prods[j].id == res[i]['Descripcion'])
        //             esta = true;

        //     if (res[i]['Descripcion'].length == 5 && !esta && typeof parseFloat(res[i]['Descripcion'][3]) == 'number') {
        //         let prod = {
        //             "id": 0,
        //             "codigos_de_barras": [],
        //             "precio_de_venta": 0,
        //             "porcentaje": 0,
        //             "precio_de_costo": 0,
        //             "tipo_producto": "",
        //             "marca": "",
        //             "variedad": "",
        //             "presentacion": {
        //                 "": 0
        //             }
        //         }
        //         prod.id = parseInt(res[i].Codigo);
        //         prod.codigos_de_barras = [parseInt(res[i]['Codigo de Barras'])]
        //         prod.precio_de_venta = res[i]['Precio de Venta'];
        //         prod.precio_de_costo = res[i]['Costo'];
        //         prod.porcentaje = ((prod.precio_de_venta / prod.precio_de_costo) - 1) * 100;
        //         prod.tipo_producto = res[i]['Descripcion'][0];
        //         prod.marca = res[i]['Descripcion'][1];
        //         prod.variedad = res[i]['Descripcion'][2];
        //         switch (res[i]['Descripcion'][4].toUpperCase()) {
        //             case 'UN':
        //                 prod.presentacion = {
        //                     'UN': parseFloat(res[i]['Descripcion'][3])
        //                 };
        //                 break;
        //             case 'LT':
        //                 prod.presentacion = {
        //                     'LT': parseFloat(res[i]['Descripcion'][3])
        //                 };
        //                 break;
        //             case 'KG':
        //                 prod.presentacion = {
        //                     'KG': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             case 'ML':
        //                 prod.presentacion = {
        //                     'ML': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             case 'CC':
        //                 prod.presentacion = {
        //                     'CC': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             case 'GR':
        //                 prod.presentacion = {
        //                     'GR': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             case 'GRS':
        //                 prod.presentacion = {
        //                     'GR': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             case 'G':
        //                 prod.presentacion = {
        //                     'GR': parseFloat(res[i]['Descripcion'][3])
        //                 }
        //                 break;
        //             default:
        //                 continue;

        //         }

        //         prods.push(prod)

        //     }
        // }




        for (let i = 0; i < res.length; i++) { //carga codigos de barras aleatorios
            if (res[i].codigos_de_barras.length > 0) {
                console.log(res[i].codigos_de_barras)
                console.log(Math.floor(Math.random() * 99999999999999))
            } else {
                res[i].codigos_de_barras.push(Math.floor(Math.random() * 99999999999999))
            }
        }
        prods = res;


        console.log(prods.length);
        console.log(JSON.stringify(prods))

        //TODO hay que parsear todo


        /*{"Codigo":"654165416516682","Descripcion":["OBLEA","ARBANIT","FRUTILLA","28","Gr"],
        "Codigo de Barras":"763331452213","Familia":"GOLOSINAS","Costo":194.66,"Precio de Venta":370,
        "Stock Minimo":0,"Tope de Descuento":100,"IVA":21,"Pesable":"N","Tipo":"P","Activo":"S",
        "Impuesto Interno":0,"Publicar en Web":"S","Unidades por bulto":0,"Dias de Stock":0,"Favorito":"N",
        "RG 5329/23":"N"}*/

        /*{
        "id": 0,
            "codigo_de_barras": 7784919681,
            "precio_de_venta": 1120.0,
            "porcentaje": 40.0,
            "precio_de_costo": 800.0,
            "tipo_producto": "Cigarrillos",
            "marca": "Marlboro",
            "variedad": "KS",
            "presentacion": {
                "Un": 20
            }
        },*/


    };
    reader.readAsBinaryString(file);

}

function displayContents(contents) {
    var element = document.getElementById('file-content');
    element.textContent = contents;
}

document.getElementById('file-input')
    .addEventListener('change', readSingleFile, false);

