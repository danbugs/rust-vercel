let button = document.querySelector('button#rust-request');

button.addEventListener('click', () => {
    let server = 'backend/index.rs'
    fetch(server, {
        method: 'GET',
        headers: {
            'Content-Type' : 'text/html'
        }
    })
    .then(response => response.text())
    .then(html => {
        let container = document.querySelector('div#rust-response');
        container.innerHTML += html;
    })
})