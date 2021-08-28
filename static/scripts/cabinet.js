'use strict';

let f = function() {
  let old = document.getElementById('old-contacts').innerHTML;
  document.getElementById('contacts').innerHTML =
    `<p>Новые контакты:</p>
    <textarea id="new-contacts" cols="30" rows="5">${old}</textarea>
    <br><br><div id="save" class="button">
        <span class="button_txt">Сохранить</span>
    </div>`;
    document.getElementById('save').onclick = async function() {
      let newc = document.getElementById('new-contacts').value;
      const res = await fetch("/api/update_contacts", {
        method: "POST",
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            login: login,
            contacts: newc,
        })
      });
      if (res.ok) {
        document.getElementById('contacts').innerHTML = `
        <b>Контакты:</b> <pre id="old-contacts">${newc}</pre>
        <br>
        <div id="change" class="button">
            <span class="button_txt">Изменить контакты</span>
        </div>
        `;
        document.getElementById('change').onclick = f;
      }
    }
};

function ready() {
    document.getElementById('change').onclick = f;
}

document.addEventListener("DOMContentLoaded", ready);