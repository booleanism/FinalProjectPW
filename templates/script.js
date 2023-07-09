function postData(uri, data_json) {
    fetch(uri, {
        method: "post",
        body: data_json,
        headers: {
            "Content-Type": "application/json"
        }
    }).then(function(respons) {
        if (respons.ok) {
            return respons.json();
        }
    });
}

function auth() {
    const formAuth = document.getElementById("form-auth");

    if (formAuth != null) {
        formAuth.addEventListener("submit", function(event) {
            event.preventDefault();
    
            const formData = new FormData(formAuth);
            const jsonString = JSON.stringify(Object.fromEntries(formData));
    
    
            console.log(jsonString);
            postData("/auth", jsonString);
        });
    }
}

function register() {
    const formLogin = document.getElementById("form-register");

    if (formLogin != null) {
        formLogin.addEventListener("submit", function(event) {
            event.preventDefault();
    
            const formData = new FormData(formLogin);
            const jsonString = JSON.stringify(Object.fromEntries(formData))
            
            postData("/reg", jsonString);
        });
    }

}

function bandCampLink() {
    const formLink = document.getElementById("form-bclink");

    if (formLink != null) {
        formLink.addEventListener("submit", function(event) {
            event.preventDefault();
    
            const formData = new FormData(formLink);
            const jsonString = JSON.stringify(Object.fromEntries(formData));
    
            postData("/bclink", jsonString)
        })
    }
}

function updatePasswd() {
    const formPass = document.getElementById('form-change-pass');

    if (formPass != null) {
        formPass.addEventListener("submit", function(event) {
            event.preventDefault();
    
            const formData = new FormData(formPass);
            const jsonString = JSON.stringify(formData)
    
            postData("/changepass", jsonString);
        });
    }
}

function main() {
    register();
    auth();
    bandCampLink();
    updatePasswd();
}

main();