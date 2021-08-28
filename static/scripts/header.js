'use strict';

function ready() {
    document.getElementById('auth').onclick = async function() {
        let login = document.getElementById('login').value;
        let password = document.getElementById('password').value;

        const res = await fetch(`/api/auth?login=${login}&password=${password}`, {
            method: "GET",
            headers: {
              'Accept': 'application/json',
              'Content-Type': 'application/json'
            }
        }).then((res) => {
            if (res.ok) {
                location.reload();
            }
        });
    }
}

document.addEventListener("DOMContentLoaded", ready);