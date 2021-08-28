'use strict';

const toBase64 = (file) =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(reader.result);
    reader.onerror = (error) => reject(error);
  });

function ready() {
    document.getElementById('add-member').onclick = function() {
      document.getElementById('team').innerHTML += 
        '<input required spellcheck="false" name="member" placeholder="логин"><br>';
    }

    document.getElementById('new-project').onclick = async function() {
      let year = Number(document.getElementById('year').value);
      let title = document.getElementById('title').value;
      let tags = document.getElementById('s-input').value.trim().split(' ');
      let descr = document.getElementById('descr').value;
      let presn = "";
      if (document.getElementById('presn').files.length > 0) {
        let f = document.getElementById('presn').files[0];
        presn = await toBase64(f).catch((e) => Error(e));
        presn = presn.slice(presn.indexOf(',') + 1);
      }
      let team = [];
      document.getElementsByName('member').forEach(el => {
        if (el.value.length > 0)
          team.push(el.value);
      });
      let mentor = document.getElementById('mentor').value;

      const res = await fetch("/api/project/new", {
          method: "POST",
          headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
              year: year,
              title: title,
              tags: tags,
              description: descr,
              presentation: presn,
              team: team,
              mentor: mentor
          })
      });
      if (res.ok) {
        let jres = await res.json();
        if (jres.id > 0)
          location.replace(`/project/${jres.id}`);
      }
    }
}

document.addEventListener("DOMContentLoaded", ready);