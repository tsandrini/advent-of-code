open Core
open Utils

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
  |> UList.fold_all_true
  |> fun x -> x && !smudge_check

let parse =
  UString.split_on_substr ~substr:"\n\n"
  >> List.map ~f:(fun mat ->
         String.split_lines mat |> List.map ~f:String.to_list)

let solver ~smudge =
  List.fold ~init:0 ~f:(fun acc mat ->
      let mat_t = UMat.transpose mat in
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

let part1 = solver ~smudge:false
let part2 = solver ~smudge:true

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day13 part1 - example data" =
  (parse >> part1)
    "#.##..##.\n\
     ..#.##.#.\n\
     ##......#\n\
     ##......#\n\
     ..#.##.#.\n\
     ..##..##.\n\
     #.#.##.#.\n\n\
     #...##..#\n\
     #....#..#\n\
     ..##..###\n\
     #####.##.\n\
     #####.##.\n\
     ..##..###\n\
     #....#..#"
  = 405

let%test "Day13 part2 - example data" =
  (parse >> part2)
    "#.##..##.\n\
     ..#.##.#.\n\
     ##......#\n\
     ##......#\n\
     ..#.##.#.\n\
     ..##..##.\n\
     #.#.##.#.\n\n\
     #...##..#\n\
     #....#..#\n\
     ..##..###\n\
     #####.##.\n\
     #####.##.\n\
     ..##..###\n\
     #....#..#"
  = 400
