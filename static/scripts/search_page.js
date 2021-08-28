'use strict';

function ready() {
    document.getElementById('start-search').onclick = async function() {
        let tags = document.getElementById('s-input').value.trim().split(' ');
        let conj = false;
        if (document.querySelector('input[name="conj"]:checked').value == 2)
            conj = true;

        let cont = document.getElementById('wrapper');
        cont.innerHTML = 'Идёт поиск';
        const res = await fetch("/api/search", {
            method: "POST",
            headers: {
              'Accept': 'application/json;charset=utf-8',
              'Content-Type': 'application/json;charset=utf-8'
            },
            body: JSON.stringify({
                tags: tags,
                conj: conj
            })
        });
        if (res.ok) {
            let jres = await res.json();
            cont.innerHTML = '';
            if (jres.projects.length == 0)
                cont.innerHTML = 'Поиск не дал результатов';
            else
            jres.projects.forEach(p => {
                let tags = '';
                p.tags.forEach(t => {
                    tags += `<div class="tag">${t}</div>`;
                });
                
                let prj = document.createElement('div');
                prj.setAttribute('class', 'project-card');
                prj.innerHTML = 
                `<b>${p.title}</b><br>
                <p class="project-card_short">${p.short}...</p>
                <div class="project-card_tags">
                    ${tags}
                </div>
                <a href="/project/${p.id}" class="project-card_more">узнать больше...</a>`;
                cont.insertBefore(prj, cont.firstChild);
            });
        }
    }
}

document.addEventListener("DOMContentLoaded", ready);