use super::{Closure, InlineScript, InlineStyle, Style};

pub fn tpl() -> String {
  let names = vec!["kev", "cass", "li"];
  let markup = html! {
    html {
      head {
        (Style("https://cdnjs.cloudflare.com/ajax/libs/animate.css/3.5.2/animate.min.css"))
        
        (InlineStyle("    
          * {
            box-sizing: border-box;    
          }    

          body {
            background-color: #51C0B0;
            font-size: 20px;
          }     

          .wrapper {
            width: 100%;
            height: 100%;
            display: flex;
          }

          .box {
            border: 3px solid white;
            padding: 20px;
            box-shadow: 0px 0px 26px 9px rgba(0,0,0,0.1);
            background-color: #EEDAC1;
            margin: auto;
          }
        "))
      }

      body {
        div class="wrapper" {
          div class="box" {
            "Click the box!"

            br /

            (Closure(&|| -> String {
              "Hello from a closure!".to_owned()
            }))
            
            @for name in names {
              p class="name" { "Hi, " (name) "!" }
            }
          }
        }

        (InlineScript("
          document.querySelector('.box').onclick = function() {
            if (this.classList.contains('animated')) {
              this.classList.remove('animated');
              this.classList.remove('bounce');
            }
            
            requestAnimationFrame(function() {
              this.classList.add('animated');
              this.classList.add('bounce');
            }.bind(this));            
          };
        "))
      }
    }
  };

  markup.into_string()
}