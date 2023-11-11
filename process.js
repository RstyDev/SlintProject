var contents;
function readSingleFile(e) {
    var file = e.target.files[0];
    if (!file) {
        return;
    }
    var reader = new FileReader();
    reader.onload = function (e) {
        var contents = e.target.result;
        let res=JSON.parse(contents);
        console.log(res[0]);
        
        //TODO hay que parsear todo




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

