const { invoke } = window.__TAURI__.core;


// elementos del DOM
let nombreInputEl;
let pesoInputEl;
let tallaInputEl;
let resultadoMsgEl;

// Funcion para calcular el IMC
async function calcularIMC() {
  const nombre = nombreInputEl.value;
  const peso = parseFloat(pesoInputEl.value);
  const talla = parseFloat(tallaInputEl.value);

  if (isNaN(peso) || isNaN(talla) || peso <= 0 || talla <= 0) {
    resultadoMsgEl.textContent = "Por favor, ingresa valores válidos.";
    return;
  }
  try {
    // Invocar la función Rust para calcular el IMC
    const mensaje = await invoke("calcular_y_guardar_imc", {
      nombre: nombre,
      peso: peso,
      talla: talla
    });
    // Mostrar el resultado en el DOM
    resultadoMsgEl.textContent = "Hola " + nombre + ", " + mensaje;
  } catch (error) {
    console.error("Error al calcular el IMC:", error);
    resultadoMsgEl.textContent = "Ocurrió un error al calcular el IMC.";
  }
}

// Cuando la pagina carga completamente

window.addEventListener("DOMContentLoaded", () => {
  // Referencias a los inputs
  nombreInputEl = document.querySelector("#nombre");
  pesoInputEl = document.querySelector("#peso");
  tallaInputEl = document.querySelector("#talla");
  resultadoMsgEl = document.querySelector("#resultado");

  // Escuchar evento de envío de formulario
  document.querySelector("#imc-form").addEventListener("submit", (event) => {
    event.preventDefault(); // Prevenir el envío del formulario
    calcularIMC(); // Llamar a la función para calcular el IMC
  }
  );

});
