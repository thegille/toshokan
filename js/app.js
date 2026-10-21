document
.getElementById("loanBtn")
.addEventListener("click", ()=>{
    const book =
    document.getElementById("book").value;
    document.getElementById("status")
    .innerText =
    Libro prestado: ${book};
});
