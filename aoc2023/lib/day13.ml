open Core

let is_horizontal_reflection ?(smudge = false) mat line =
  let len = List.length mat in

  let smudge_check = ref (not smudge) in
  let check i j =
    let comparisons =
      List.zip_exn (List.nth_exn mat i) (List.nth_exn mat j)
      |> List.map ~f:(fun (a, b) -> Char.equal a b)
      |> List.filter ~f:(fun x -> not x)
      |> List.length
    in
    if comparisons = 0 then true
    else if comparisons = 1 && not !smudge_check then (
      smudge_check := true;
      true)
    else false
  in

  List.init (line + 1) ~f:(fun i ->
      if line + i + 1 >= len then true
      else if check (line - i) (line + i + 1) then true
      else false)
  |> Utils.list_all_true
  |> fun x -> x && !smudge_check

let parse input =
  Utils.split_on_substring ~substring:"\n\n" input
  |> List.map ~f:(fun mat ->
         String.split_lines mat |> List.map ~f:String.to_list)

let solve matrices =
  let solver ~smudge =
    List.fold matrices ~init:0 ~f:(fun acc mat ->
        let mat_t = Utils.transpose mat in
        let horizontal_width =
          List.init (List.length mat - 1) ~f:(fun i -> i)
          |> List.filter ~f:(is_horizontal_reflection ~smudge mat)
          |> fun lst -> match List.hd lst with Some x -> x + 1 | None -> 0
        in
        let vertical_width =
          List.init (List.length mat_t - 1) ~f:(fun i -> i)
          |> List.filter ~f:(is_horizontal_reflection ~smudge mat_t)
          |> fun lst -> match List.hd lst with Some x -> x + 1 | None -> 0
        in
        acc + vertical_width + (100 * horizontal_width))
  in
  Printf.printf "Part1: %d \n" (solver ~smudge:false);
  Printf.printf "Part2: %d \n" (solver ~smudge:true)

let main input = In_channel.read_all input |> parse |> solve
