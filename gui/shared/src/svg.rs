#![allow(dead_code)]

const ICON_FILTER: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-filter" viewBox="0 0 16 16">
  <path d="M6 10.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 0 1h-3a.5.5 0 0 1-.5-.5m-2-3a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5m-2-3a.5.5 0 0 1 .5-.5h11a.5.5 0 0 1 0 1h-11a.5.5 0 0 1-.5-.5"/>
</svg>
"##;

const ICON_INSTALL: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-arrow-down-square-fill" viewBox="0 0 16 16">
  <path d="M2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2zm6.5 4.5v5.793l2.146-2.147a.5.5 0 0 1 .708.708l-3 3a.5.5 0 0 1-.708 0l-3-3a.5.5 0 1 1 .708-.708L7.5 10.293V4.5a.5.5 0 0 1 1 0"/>
</svg>
"##;

const ICON_REFRESH: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"/>
  <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466"/>
</svg>
"##;

const ICON_HOME: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-house-fill" viewBox="0 0 16 16">
  <path d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L8 2.207l6.646 6.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293z"/>
  <path d="m8 3.293 6 6V13.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5V9.293z"/>
</svg>
"##;

const ICON_BACKGROUND: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" class="bi bi-play-fill" viewBox="0 0 30 20">
  <rect width="100%" height="100%" fill="#2A2E32" opacity="0.65"></rect>
</svg>
"##;

const ICON_CONFIGURE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-gear-fill" viewBox="0 0 16 16">
  <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"/>
</svg>
"#;

const ICON_BACK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8"/>
</svg>
"#;

const ICON_PLAY: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"/>
</svg>
"#;

const ICON_LIST: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"/>
</svg>
"#;

const ICON_ADD: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-plus-lg" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M8 2a.5.5 0 0 1 .5.5v5h5a.5.5 0 0 1 0 1h-5v5a.5.5 0 0 1-1 0v-5h-5a.5.5 0 0 1 0-1h5v-5A.5.5 0 0 1 8 2"/>
</svg>
"#;

const ICON_DEL: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-trash2-fill" viewBox="0 0 16 16">
  <path d="M2.037 3.225A.7.7 0 0 1 2 3c0-1.105 2.686-2 6-2s6 .895 6 2a.7.7 0 0 1-.037.225l-1.684 10.104A2 2 0 0 1 10.305 15H5.694a2 2 0 0 1-1.973-1.671zm9.89-.69C10.966 2.214 9.578 2 8 2c-1.58 0-2.968.215-3.926.534-.477.16-.795.327-.975.466.18.14.498.307.975.466C5.032 3.786 6.42 4 8 4s2.967-.215 3.926-.534c.477-.16.795-.327.975-.466-.18-.14-.498-.307-.975-.466z"/>
</svg>
"#;

const ICON_CLOSE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-x-square-fill" viewBox="0 0 16 16">
  <path d="M2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2zm3.354 4.646L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 1 1 .708-.708"/>
</svg>
"#;

const ICON_SWITCH: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-arrow-left-right" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M1 11.5a.5.5 0 0 0 .5.5h11.793l-3.147 3.146a.5.5 0 0 0 .708.708l4-4a.5.5 0 0 0 0-.708l-4-4a.5.5 0 0 0-.708.708L13.293 11H1.5a.5.5 0 0 0-.5.5m14-7a.5.5 0 0 1-.5.5H2.707l3.147 3.146a.5.5 0 1 1-.708.708l-4-4a.5.5 0 0 1 0-.708l4-4a.5.5 0 1 1 .708.708L2.707 4H14.5a.5.5 0 0 1 .5.5"/>
</svg>
"#;

const ICON_JOYSTICK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-joystick" viewBox="0 0 16 16">
  <path d="M10 2a2 2 0 0 1-1.5 1.937v5.087c.863.083 1.5.377 1.5.726 0 .414-.895.75-2 .75s-2-.336-2-.75c0-.35.637-.643 1.5-.726V3.937A2 2 0 1 1 10 2"/>
  <path d="M0 9.665v1.717a1 1 0 0 0 .553.894l6.553 3.277a2 2 0 0 0 1.788 0l6.553-3.277a1 1 0 0 0 .553-.894V9.665c0-.1-.06-.19-.152-.23L9.5 6.715v.993l5.227 2.178a.125.125 0 0 1 .001.23l-5.94 2.546a2 2 0 0 1-1.576 0l-5.94-2.546a.125.125 0 0 1 .001-.23L6.5 7.708l-.013-.988L.152 9.435a.25.25 0 0 0-.152.23"/>
</svg>
"#;

const ICON_SAVE: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-floppy2-fill" viewBox="0 0 16 16">
  <path d="M12 2h-2v3h2z"/>
  <path d="M1.5 0A1.5 1.5 0 0 0 0 1.5v13A1.5 1.5 0 0 0 1.5 16h13a1.5 1.5 0 0 0 1.5-1.5V2.914a1.5 1.5 0 0 0-.44-1.06L14.147.439A1.5 1.5 0 0 0 13.086 0zM4 6a1 1 0 0 1-1-1V1h10v4a1 1 0 0 1-1 1zM3 9h10a1 1 0 0 1 1 1v5H2v-5a1 1 0 0 1 1-1"/>
</svg>
"#;

const ICON_BOX_HEART: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-box2-heart-fill" viewBox="0 0 16 16">
  <path d="M3.75 0a1 1 0 0 0-.8.4L.1 4.2a.5.5 0 0 0-.1.3V15a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V4.5a.5.5 0 0 0-.1-.3L13.05.4a1 1 0 0 0-.8-.4zM8.5 4h6l.5.667V5H1v-.333L1.5 4h6V1h1zM8 7.993c1.664-1.711 5.825 1.283 0 5.132-5.825-3.85-1.664-6.843 0-5.132"/>
</svg>
"#;

const ICON_CLOUD: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-cloud-download-fill" viewBox="0 0 16 16">
  <path fill-rule="evenodd" d="M8 0a5.53 5.53 0 0 0-3.594 1.342c-.766.66-1.321 1.52-1.464 2.383C1.266 4.095 0 5.555 0 7.318 0 9.366 1.708 11 3.781 11H7.5V5.5a.5.5 0 0 1 1 0V11h4.188C14.502 11 16 9.57 16 7.773c0-1.636-1.242-2.969-2.834-3.194C12.923 1.999 10.69 0 8 0m-.354 15.854a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 14.293V11h-1v3.293l-2.146-2.147a.5.5 0 0 0-.708.708z"/>
</svg>
"#;

const ICON_FOLDER: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-folder-fill" viewBox="0 0 16 16">
  <path d="M9.828 3h3.982a2 2 0 0 1 1.992 2.181l-.637 7A2 2 0 0 1 13.174 14H2.825a2 2 0 0 1-1.991-1.819l-.637-7a2 2 0 0 1 .342-1.31L.5 3a2 2 0 0 1 2-2h3.672a2 2 0 0 1 1.414.586l.828.828A2 2 0 0 0 9.828 3m-8.322.12q.322-.119.684-.12h5.396l-.707-.707A1 1 0 0 0 6.172 2H2.5a1 1 0 0 0-1 .981z"/>
</svg>
"#;

const ICON_CHECK: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" class="bi bi-check-lg" viewBox="0 0 16 16">
  <path d="M12.736 3.97a.733.733 0 0 1 1.047 0c.286.289.29.756.01 1.05L7.88 12.01a.733.733 0 0 1-1.065.02L3.217 8.384a.757.757 0 0 1 0-1.06.733.733 0 0 1 1.047 0l3.052 3.093 5.4-6.425z"/>
</svg>
"#;

const ICON_BOX_SELECTED: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="#006D00" class="bi bi-check-square-fill" viewBox="0 0 16 16">
  <path d="M 0,0 -0.0077,0.005731 V 16 H -0.0113 15.99234 V 16.000193 0 h 9.68e-4 z m 12.022255,4.97 c 0.288934,0.2886287 0.293824,0.7553814 0.011,1.05 l -3.992,4.99 c -0.289263,0.311563 -0.7794,0.32064 -1.08,0.02 l -2.645,-2.646 c -0.757891,-0.7062104 0.35379,-1.8178912 1.06,-1.06 l 2.094,2.093 3.473,-4.425 c 0.288689,-0.3121252 0.778839,-0.3221097 1.08,-0.022 z" id="path1" sodipodi:nodetypes="ccccccccccccccccccc" />
</svg>
"##;

const ICON_BOX_DESELECTED: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="#8E3838" class="bi bi-x-square-fill" viewBox="0 0 16 16">
  <path d="m 0,0 -0.0023,0.002837 v 15.998933 -0.0018 h 15.996765 l 0.0032,-0.0098 V 0.00369266 l -0.003,-0.003693 z M 5.351742,4.646 7.9977415,7.293 10.643742,4.646 c 0.471999,-0.4719998 1.179999,0.2360002 0.707999,0.708 L 8.7047415,8 11.351742,10.646 c 0.47165,0.472 -0.23635,1.18 -0.708,0.708 L 7.9977415,8.707 5.351742,11.354 c -0.472,0.471651 -1.18,-0.236349 -0.708,-0.708 L 7.290742,8 4.643742,5.354 c -0.472,-0.4719998 0.236,-1.1799998 0.708,-0.708" id="path1" sodipodi:nodetypes="cccccccccccccccccccccc" />
</svg>
"##;

const ICON_HAMBURGUER: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" fill="white" viewBox="0 0 16 16">
  <path d="M1 0 0 1l2.2 3.081a1 1 0 0 0 .815.419h.07a1 1 0 0 1 .708.293l2.675 2.675-2.617 2.654A3.003 3.003 0 0 0 0 13a3 3 0 1 0 5.878-.851l2.654-2.617.968.968-.305.914a1 1 0 0 0 .242 1.023l3.27 3.27a.997.997 0 0 0 1.414 0l1.586-1.586a.997.997 0 0 0 0-1.414l-3.27-3.27a1 1 0 0 0-1.023-.242L10.5 9.5l-.96-.96 2.68-2.643A3.005 3.005 0 0 0 16 3c0-.269-.035-.53-.102-.777l-2.14 2.141L12 4l-.364-1.757L13.777.102a3 3 0 0 0-3.675 3.68L7.462 6.46 4.793 3.793a1 1 0 0 1-.293-.707v-.071a1 1 0 0 0-.419-.814zm9.646 10.646a.5.5 0 0 1 .708 0l2.914 2.915a.5.5 0 0 1-.707.707l-2.915-2.914a.5.5 0 0 1 0-.708M3 11l.471.242.529.026.287.445.445.287.026.529L5 13l-.242.471-.026.529-.445.287-.287.445-.529.026L3 15l-.471-.242L2 14.732l-.287-.445L1.268 14l-.026-.529L1 13l.242-.471.026-.529.445-.287.287-.445.529-.026z"/>
</svg>
"#;

pub const ICON_GAMEIMAGE: &str = r##"
<svg
   width="127.1859mm"
   height="124.66522mm"
   viewBox="0 0 127.1859 124.66522"
   version="1.1"
   id="svg5"
   inkscape:version="1.2.1 (9c6d41e410, 2022-07-14)"
   sodipodi:docname="gameimage.svg"
   xml:space="preserve"
   inkscape:export-filename="gameimage.png"
   inkscape:export-xdpi="524.22369"
   inkscape:export-ydpi="524.22369"
   xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
   xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg"><sodipodi:namedview
     id="namedview7"
     pagecolor="#ffffff"
     bordercolor="#666666"
     borderopacity="1.0"
     inkscape:showpageshadow="2"
     inkscape:pageopacity="0.0"
     inkscape:pagecheckerboard="0"
     inkscape:deskcolor="#d1d1d1"
     inkscape:document-units="mm"
     showgrid="false"
     inkscape:zoom="0.76257019"
     inkscape:cx="241.28927"
     inkscape:cy="189.49075"
     inkscape:window-width="1268"
     inkscape:window-height="1000"
     inkscape:window-x="0"
     inkscape:window-y="0"
     inkscape:window-maximized="1"
     inkscape:current-layer="layer1"
     showguides="true" /><defs
     id="defs2"><linearGradient
       id="linearGradient43840"
       inkscape:swatch="solid"><stop
         style="stop-color:#977346;stop-opacity:1;"
         offset="0"
         id="stop43838" /></linearGradient></defs><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(32.429114,-803.7911)"><rect
       style="fill:none;fill-opacity:1;stroke:none;stroke-width:0.141751;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect27157-3"
       width="127.1859"
       height="124.66515"
       x="-32.429115"
       y="803.79114"
       rx="15.337074"
       ry="17.509348"
       inkscape:export-filename="../doc/gameimage.png"
       inkscape:export-xdpi="524.22369"
       inkscape:export-ydpi="524.22369" /><rect
       style="fill:#3771c8;fill-opacity:1;stroke:none;stroke-width:0.310604;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-2-6-9-5-0-4-7-6"
       width="18.91713"
       height="67.035065"
       x="-832.73169"
       y="16.81229"
       rx="0"
       ry="0"
       transform="rotate(-90)"
       inkscape:export-filename="../doc/gameimage.svg"
       inkscape:export-xdpi="524.22369"
       inkscape:export-ydpi="524.22369" /><rect
       style="fill:#3771c8;fill-opacity:1;stroke:none;stroke-width:0.283559;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-3-2"
       width="55.869629"
       height="18.91712"
       x="-43.808434"
       y="-918.12695"
       rx="0"
       ry="0"
       transform="scale(-1)" /><rect
       style="fill:#4d4d4d;fill-opacity:1;stroke:none;stroke-width:0.240667;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-86-1"
       width="18.91713"
       height="40.24577"
       x="-918.12695"
       y="43.808434"
       rx="0"
       ry="0"
       transform="rotate(-90)" /><rect
       style="fill:#3771c8;fill-opacity:1;stroke:none;stroke-width:0.177029;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-2-6-97-0-6-7"
       width="21.776005"
       height="18.91712"
       x="-65.137085"
       y="-874.73767"
       rx="0"
       ry="0"
       transform="scale(-1)" /><rect
       style="fill:#4d4d4d;fill-opacity:1;stroke:none;stroke-width:0.275785;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-6-48-7"
       width="52.847858"
       height="18.91712"
       x="-908.6684"
       y="65.137085"
       rx="0"
       ry="0"
       transform="rotate(-90)" /><rect
       style="fill:#d35f5f;fill-opacity:1;stroke:none;stroke-width:0.176363;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-6-48-0-9-5-5-2-9"
       width="21.612543"
       height="18.91712"
       x="-43.36108"
       y="-874.73767"
       rx="0"
       ry="0"
       transform="scale(-1)" /><rect
       style="fill:#4d4d4d;fill-opacity:1;stroke:none;stroke-width:0.241514;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-2-6-97-3"
       width="40.529617"
       height="18.91712"
       x="-19.009857"
       y="-832.70764"
       rx="0"
       ry="0"
       transform="scale(-1)" /><rect
       style="fill:#4d4d4d;fill-opacity:1;stroke:none;stroke-width:0.280122;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-6-48-0-2-6"
       width="18.91713"
       height="54.52346"
       x="2.6026304"
       y="-877.77252"
       rx="0"
       ry="0"
       transform="scale(-1)" /><rect
       style="fill:#3771c8;fill-opacity:1;stroke:none;stroke-width:0.240992;stroke-linecap:butt;stroke-linejoin:miter;stroke-dasharray:none;stroke-opacity:0.311985"
       id="rect40555-3-9-2-0-6-8-9-3-6"
       width="18.91713"
       height="40.354412"
       x="2.6026313"
       y="-918.12695"
       rx="0"
       ry="0"
       transform="scale(-1)" /></g></svg>
"##;

macro_rules! icon
{
  ($func_name:ident, $icon:expr, $size_1:expr, $size_2:expr) =>
  {
    pub fn $func_name(scale : f32) -> String
    {
      let size_1 = ($size_1 as f32 * scale) as i32;
      let str_size_1 = size_1.to_string();

      let size_2 = ($size_2 as f32 * scale) as i32;
      let str_size_2 = size_2.to_string();

      let mut result = $icon.replacen("{}", str_size_1.as_str(), 1);
      result = result.replacen("{}", str_size_2.as_str(), 1);
      result
    }
  }
}

macro_rules! icon_with_size
{
  ($func_name:ident, $icon:expr) =>
  {
    pub fn $func_name(size_1 : i32, size_2 : i32) -> String
    {
      let str_size_1 = size_1.to_string();
      let str_size_2 = size_2.to_string();
      let mut result = $icon.replacen("{}", str_size_1.as_str(), 1);
      result = result.replacen("{}", str_size_2.as_str(), 1);
      result
    }
  }
}


icon!(icon_filter, ICON_FILTER, 16, 16);
icon!(icon_install, ICON_INSTALL, 16, 16);
icon!(icon_refresh, ICON_REFRESH, 16, 16);
icon!(icon_home, ICON_HOME, 16, 16);
icon!(icon_background, ICON_BACKGROUND, 317, 60);
icon!(icon_configure, ICON_CONFIGURE, 16, 16);
icon!(icon_back, ICON_BACK, 20, 20);
icon!(icon_play, ICON_PLAY, 28, 28);
icon!(icon_list, ICON_LIST, 16, 16);
icon!(icon_add, ICON_ADD, 24, 24);
icon!(icon_del, ICON_DEL, 16, 16);
icon!(icon_joystick, ICON_JOYSTICK, 16, 16);
icon!(icon_save, ICON_SAVE, 16, 16);
icon!(icon_cloud, ICON_CLOUD, 16, 16);
icon!(icon_box_heart, ICON_BOX_HEART, 16, 16);
icon!(icon_folder, ICON_FOLDER, 16, 16);
icon!(icon_check, ICON_CHECK, 24, 24);
icon!(icon_switch, ICON_SWITCH, 16, 16);
icon!(icon_close, ICON_CLOSE, 16, 16);
icon!(icon_hamburguer, ICON_HAMBURGUER, 24, 24);
icon!(icon_box_selected, ICON_BOX_SELECTED, 16, 16);
icon!(icon_box_deselected, ICON_BOX_DESELECTED, 16, 16);

pub mod with_size
{

icon_with_size!(icon_box_deselected, crate::svg::ICON_BOX_DESELECTED);
icon_with_size!(icon_box_selected, crate::svg::ICON_BOX_SELECTED);

}

// vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
