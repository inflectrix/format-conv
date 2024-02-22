import * as FmtConv from "format_conv";

function updateTextboxes(data) {
    const converted = FmtConv.convert_to_all(data);

    document.getElementById("jsonbox").innerText = converted.json;
    document.getElementById("tomlbox").innerText = converted.toml;
    document.getElementById("ronbox").innerText = converted.ron;
    document.getElementById("yamlbox").innerText = converted.yaml;
    document.getElementById("qsbox").innerText = converted.qstr;
    document.getElementById("lexprbox").innerText = converted.lexpr;
}   

function desJson() {
    const s = document.getElementById("jsonbox").innerText;
    const data = FmtConv.des_json(s);

    updateTextboxes(data);    
}

function desToml() {
    const s = document.getElementById("tomlbox").innerText;
    const data = FmtConv.des_toml(s);

    updateTextboxes(data);    
}

function desRon() {
    const s = document.getElementById("ronbox").innerText;
    const data = FmtConv.des_ron(s);

    updateTextboxes(data);    
}

function desYaml() {
    const s = document.getElementById("yamlbox").innerText;
    const data = FmtConv.des_yaml(s);

    updateTextboxes(data);    
}

function desQS() {
    const s = document.getElementById("qsbox").innerText;
    const data = FmtConv.des_qs(s);

    updateTextboxes(data);    
}

function desLexpr() {
    const s = document.getElementById("lexprbox").innerText;
    const data = FmtConv.des_lexpr(s);

    updateTextboxes(data);    
}