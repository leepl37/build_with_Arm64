function get_footer(name) {
    let play_status_footer = document.getElementById("play_and_stop_status_footer");
    let playlist_footer = document.getElementById("playlist_footer");
    let filelist_footer = document.getElementById("filelist_footer");
    playlist_footer.style.display = "none";
    filelist_footer.style.display = "none";
    play_status_footer.style.display = "none";
    if(name.startsWith("playlist_footer")){
        playlist_footer.style.display = "block";
    }else if(name.startsWith("play_and_stop_status_footer")){
        play_status_footer.style.display = "block";
    }else if(name.startsWith("filelist_footer")){
        filelist_footer.style.display = "block";
    }
}


function screenshot() {
    let strWindowFeatures = "width=250,height=230,menubar=yes,location=yes,resizable=yes,scrollbars=yes,status=yes";
    window.open("http://172.30.1.45:8000/api/screenshot", "현재 재생 중인 파일 화면","width=250,height=200,toolbal=no,titlebal=no,status=no")
}

function close_playlist() {
    let playlist_forms = document.getElementsByClassName("playlist_name_form");
    for(let i=0; i < playlist_forms.length; i++){
        playlist_forms[i].style.display = "none";
    }
}

function check_play_or_stop_then_change_the_td_inner_text(name) {
    let current_playlist_name = document.getElementById("current_playlist_name");
    let play_or_stop_name_tag = document.getElementById("play_or_stop_name_tag");
    if(current_playlist_name.innerText.startsWith(name)){
        var img = document.createElement("img").src = "assets/static/img/stop_32px.png";
        play_or_stop_name_tag.innerHTML = "";
        play_or_stop_name_tag.innerHTML = "<img src='assets/static/img/stop_32px.png'><span style='vertical-align: super;margin-left: 10px;'>정지</span>";
    }else{
        play_or_stop_name_tag.innerHTML = "<img src='assets/static/img/play_32px.png'><span style='vertical-align: super;margin-left: 10px;'>재생</span>";
    }
}

function click_filelist_or_playlist(name){
    close_playlist();
    if(name.startsWith("filelist")){
        get_footer("filelist_footer");
        let playlist_form = document.getElementById("playlist_form");
        playlist_form.style.display = "none";
        document.getElementById("filelist_th").style.color = "blue";
        document.getElementById("playlist_th").style.color = "black";
        let filelist_form = document.getElementById("filelist_form");
        filelist_form.style.display = "block";
    }else{
        get_footer("playlist_footer");
        let filelist_form = document.getElementById("filelist_form");
        filelist_form.style.display = "none";
        document.getElementById("filelist_th").style.color = "black";
        document.getElementById("playlist_th").style.color = "blue";
        let playlist_form = document.getElementById("playlist_form");
        playlist_form.style.display = "block";
    }
}


let before_clicked = "";
function click_playlist_name(name) {
    let playlist_name = document.getElementById("clicked_playlist_name");
    playlist_name.value = name;

    let current_playlist_name = document.getElementById("current_playlist_name");
    let name_tag_button = document.getElementById("play_or_stop_name_tag");
    console.log("name:" + name);
    console.log("current_name:" + current_playlist_name.innerText);
    check_play_or_stop_then_change_the_td_inner_text(name);
    if(name.startsWith(current_playlist_name.innerText.trim())){
        let status = document.getElementById("play_status");
        if (status.innerText.startsWith("정지")) {
            name_tag_button.innerHTML = "<img src='assets/static/img/play_32px.png'><span style='vertical-align: super;margin-left: 10px;'>재생</span>"
        }else{
            name_tag_button.innerHTML = "<img src='assets/static/img/stop_32px.png'><span style='vertical-align: super;margin-left: 10px;'>정지</span>"
        }
    }else{
        name_tag_button.innerHTML = "<img src='assets/static/img/play_32px.png'><span style='vertical-align: super;margin-left: 10px;'>재생</span>"
    }
    
    get_footer("play_and_stop_status_footer");
    close_playlist();

    let playlist_form = document.getElementById(name);

    if(before_clicked.startsWith(name)){
        playlist_form.style.display = "none";
        get_footer("playlist_footer");
        before_clicked = "";
    }else{
        playlist_form.style.display = "block";
        before_clicked = name;
    }
}
function showProCircle() {
    document.getElementById("outer_div").style.pointerEvents = "none";
    // document.getElementById("outer_div").style.backgroundColor = "black";
    document.getElementById("outer_div").style.opacity = "0.4";

    document.getElementsByClassName("loadingBox")[0].style.display = "block";
}

function check_file_selected() {
    let t = document.getElementById("file_name_column").files.length
    if(t >= 1){
        let result = confirm("파일을 업로드 하시겠습니까?");
        if(result){
            showProCircle();
            document.getElementById("file_upload_form").submit()
            // play_or_stop_name_tag.innerHTML = "<img src='assets/static/img/stop_32px.png'><span style='vertical-align: super;margin-left: 10px;'>정지</span>";
        }else{

        }
    }else{
    }
}

function file_upload_new_browser(){
    document.getElementById("file_name_column").click();
}

function file_delete(field_name) {
    let file_lists = document.getElementsByClassName(field_name);
    let file_name_to_delete = [];
    let number =0;
    for(let i=0; i < file_lists.length; i++){
        if(file_lists[i].style.backgroundColor.startsWith("aliceblue")){
            number+=1;
            console.log("총" + number + "개의 파일을 선택하셨습니다.")
            file_name_to_delete.push(file_lists[i].id);
            console.log(file_name_to_delete)
        }
    }
    if(number < 1) {
        alert("파일을 선택해주신 후 삭제버튼을 클릭해주세요.");
    }else{
        if(confirm(number + "개의 파일을 삭제하시겠습니까?")){
            let deleteform = "";
            if(field_name.startsWith("file_name_list_table_td")){
                deleteform = document.getElementById("file_delete_form");
            }else{
                deleteform = document.getElementById("file_name_in_playlist_delete_form");
            }
            deleteform.file_name.value = file_name_to_delete;
            console.log(deleteform);

            deleteform.submit()
        }
    }
}

function file_name_list_checked(name) {
    let filename = document.getElementById(name);
    let filename_header = document.getElementById(name+"_header");
//    if(filename.style.backgroundColor.startsWith("gray")){
    if(filename.style.backgroundColor.startsWith("aliceblue")){
        filename_header.style.backgroundColor = "white";
        filename.style.backgroundColor = "white";
    }else{
//        filename.style.backgroundColor = "gray";
        filename_header.style.backgroundColor = "aliceblue";
        filename.style.backgroundColor = "aliceblue";
    }
}

function all_check(kind) {
    let kind_checked = "";
    if(kind.startsWith("create")){
        kind_checked = "create";
    }else if(kind.startsWith("add")){
        kind_checked = "add";
    }else{
        kind_checked = "edit";
    }
    let allChecked = "all_checked_" + kind_checked;
    let all_checked = document.getElementById(allChecked);
    console.log(all_checked.checked);

    let kind_class_name = kind_checked + "_form_files";
    if(all_checked.checked){
        let files = document.getElementsByClassName(kind_class_name);
        for(let i=0; i<files.length; i++){
            files[i].checked = true;
        }
    }else{
        let files = document.getElementsByClassName(kind_class_name);
        for(let i=0; i<files.length; i++){
            files[i].checked = false;
        }
    }
}

function is_file_checked(class_name) {
    let string = "";
    let check_file_selected = document.getElementsByClassName(class_name);
    let s = "";
    for(let i =0; i < check_file_selected.length; i++){
        if(check_file_selected[i].checked){
            let front = "";
            if(class_name.startsWith("create")){
                front="create";
                s = front + check_file_selected[i].value;
            }else if(class_name.startsWith("add")){
                front="add";
                s = front + check_file_selected[i].value;
            }else{
                front = "edit";
                s = i+1;
            }
            let file_count = document.getElementById(s);
            string += check_file_selected[i].value +":"+file_count.value +","
        }
    }
    return string
}


function save(class_name) {
    let string = "";
    let checked_file = is_file_checked(class_name);
    console.log("확인:"+checked_file)
    // if(class_name.startsWith("edit_form_files")){
    // }else {
    // }
    let form = "";
    console.log(form);
    if(class_name.startsWith("edit")){
        form = document.getElementById("add_and_edit_form")
        form.form_type.value = "edit";
    }else if(class_name.startsWith("add")){
        form = document.getElementById("add_and_edit_form")
        form.form_type.value = "add";
    }else{
        form = document.getElementById("create_form");
        form.form_type.value = "create";
    }
    form.checked_file.value = is_file_checked(class_name);

    if(form.playlist_name.value.length < 1){
        console.log(form.playlist_name.value)
        alert("플레이리스트 이름을 입력해주세요.");
    }else{
        let playlists = document.getElementsByClassName("playlist_name_from_create_form");
        let which = true;
        for(let i=0; i<playlists.length; i++){
            let playlistName = form.playlist_name.value.trim();
            let playlists_check = playlists[i].innerHTML.trim();
            if(playlists_check.startsWith(playlistName)){
                alert("동일한 이름의 플레이리스트가 존재합니다.");
                which = false;
            }
        }
        if(which){
            form.submit();
            // console.log("폼 제출")
        }
    }
}

function cancel() {
    // history.back();
    history.go(-1);
}

function plus(name){
    let file = document.getElementById(name);
    file.value = parseInt(file.value) + 1;
}

function minus(name){
    let file = document.getElementById(name);
    file.value = parseInt(file.value)-1;
}

function play_stop_edit_delete(innerText) {
    let playlist_name = document.getElementById("clicked_playlist_name").value;
    let status = innerText;
    let form = document.footer_form;
    form.playlist_name.value = playlist_name;
    form.status.value = status;

    if (innerText.startsWith("재생")) {
        let answer = confirm("플레이리스트 " + playlist_name + "을 재생하겠습니까?");
        if (answer) {
            form.submit();
        }
    } else if(innerText.startsWith("문구")){
        let answer = confirm("플레이리스트 " + playlist_name + "의 문구를 입력하겠습니까?");
        if (answer) {
            form.submit();
        }
    }else if(innerText.startsWith("정지")){
    let answer = confirm("플레이리스트 "+playlist_name + "을 정지하시겠습니까?");
    if(answer){
        form.submit();
    }
    }else if(innerText.startsWith("수정")){
        let answer = confirm("플레이리스트 "+playlist_name + "을 수정하겠습니까?");
        if(answer){
            form.submit();
        }
    }else if(innerText.startsWith("삭제")){
        let answer = confirm("플레이리스트 "+playlist_name + "을 삭제하겠습니까?");
        if(answer){
            form.submit();
        }
    }
}


function order_edit() {
    //등록할 파일을 선택해주세요 change
    let checked_add = document.getElementById("all_checked_add");
    checked_add.style.display = "none";
    let checked_edit = document.getElementById("all_checked_edit");
    checked_edit.style.display = "block";

    //form change
    let all_file_form = document.getElementById("all_file_table_from_edit");
    let files_in_playlist_form = document.getElementById("file_in_playlist_table_from_edit");
    all_file_form.style.display = "none";
    files_in_playlist_form.style.display = "block";

    //footer change
    let add_file_form = document.getElementById("add_file_to_playlist_form");
    add_file_form.style.display = "none";
    let edit_form = document.getElementById("edit_file_for_playlist_form");
    edit_form.style.display = "block";
}

function before_to() {
    //등록할 파일을 선택해주세요 change //add
    let checked_edit = document.getElementById("all_checked_edit");
    checked_edit.style.display = "none";
    let checked_add = document.getElementById("all_checked_add");
    checked_add.style.display = "block";


    let all_file_form = document.getElementById("all_file_table_from_edit");
    let files_in_playlist_form = document.getElementById("file_in_playlist_table_from_edit");
    files_in_playlist_form.style.display = "none";
    all_file_form.style.display = "block";

    let edit_form = document.getElementById("edit_file_for_playlist_form");
    edit_form.style.display = "none";
    let add_file_form = document.getElementById("add_file_to_playlist_form");
    add_file_form.style.display = "block";
}

