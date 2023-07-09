function postData(uri, data_json) {
    fetch(uri, {
        method: "post",
        body: data_json,
        headers: {
            "Content-Type": "application/json"
        }
    })
    .then(responsbody => responsbody.text())
    .then(data => {
        document.documentElement.innerHTML = data;
    });
}

function postDataLog(uri, data_json) {
    fetch(uri, {
        method: "post",
        body: data_json,
        headers: {
            "Content-Type": "application/json"
        }
    }).then(async function(response) {
        if (response.status === 202) {
          window.location.href = '/';
        } else if (response.status === 401) {
          await new Promise(resolve => setTimeout(resolve, 1000)); // Tunggu 1 detik
          location.reload();
        } else {
          // Tangani kondisi respons lainnya
        }
      })
      .catch(function(error) {
        // Tangani kesalahan jaringan atau kesalahan lainnya
        console.log(error);
      });
    // .then(responsbody => responsbody.text())
    // .then(data => {
    //     document.documentElement.innerHTML = data;
    // });
}

function postDataReg(uri, data_json) {
    fetch(uri, {
        method: "post",
        body: data_json,
        headers: {
            "Content-Type": "application/json"
        }
    }).then(async function(response) {
        if (response.status === 201) {
          window.location.href = '/login.html';
        } else if (response.status === 400) {
          await new Promise(resolve => setTimeout(resolve, 1000)); // Tunggu 1 detik
          location.reload();
        } else {
          // Tangani kondisi respons lainnya
        }
      })
      .catch(function(error) {
        // Tangani kesalahan jaringan atau kesalahan lainnya
        console.log(error);
      });
}

function postUpdatePass(uri, data_json) {
    fetch(uri, {
        method: "post",
        body: data_json,
        headers: {
            "Content-Type": "application/json"
        }
    }).then(async function(response) {
        if (response.status === 200) {
          window.location.href = '/login.html';
        } else if (response.status === 409) {
          await new Promise(resolve => setTimeout(resolve, 1000)); // Tunggu 1 detik
          location.reload();
        } else {
          // Tangani kondisi respons lainnya
        }
      })
      .catch(function(error) {
        // Tangani kesalahan jaringan atau kesalahan lainnya
        console.log(error);
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
            postDataLog("/auth", jsonString);
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
            
            postDataReg("/reg", jsonString);
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
    
            console.log(jsonString);
            postData("/bclink", jsonString);
        })
    }
}

function updatePasswd() {
    const formPass = document.getElementById('form-change-pass');

    if (formPass != null) {
        formPass.addEventListener("submit", function(event) {
            event.preventDefault();
    
            const formData = new FormData(formPass);
            const jsonString = JSON.stringify(Object.fromEntries(formData))
    
            console.log(jsonString);
            postUpdatePass("/changepass", jsonString);
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