'use strict';

function ready() {
    document.getElementById('send-comment').onclick = async function() {
        let content = document.getElementById('comment').value;
        const res = await fetch("/api/comment/new", {
            method: "POST",
            headers: {
              'Accept': 'application/json;charset=utf-8',
              'Content-Type': 'application/json;charset=utf-8'
            },
            body: JSON.stringify({
                content: content,
                pid: id
            })
        }).then((res) => {
            if (res.ok) {
                let date = new Date(Date.now()).toLocaleString('en-GB', { dateStyle: 'short', timeStyle: 'short' });
                date = date.replaceAll('/', '.');
                date = date.replace(',', '');
                
                let cmt = document.createElement('div');
                cmt.setAttribute('class', 'comment');
                cmt.innerHTML = 
                `<div class="comment_head">
                    <b>${usr_name}</b>
                    <i>${date}</i>
                </div>
                <p>${content}</p>`;
                let cont = document.getElementById('cmt-container');
                cont.insertBefore(cmt, cont.firstChild);
            }
        });
    }
}

document.addEventListener("DOMContentLoaded", ready);