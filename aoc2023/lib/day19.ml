open Core
open Utils

let nth_flip = flip List.nth_exn

let get_cat = function
  | 'x' -> nth_flip 0
  | 'm' -> nth_flip 1
  | 'a' -> nth_flip 2
  | 's' -> nth_flip 3
  | _ -> failwith "Invalid category"

let parse_rule str =
  if not (String.contains str ':') then (str, fun _ -> true)
  else
    let str', target = String.split ~on:':' str |> UList.to_tuple_exn in
    let sep = if String.contains str' '<' then '<' else '>' in
    let op = if String.contains str' '<' then ( < ) else ( > ) in
    let id, arg = String.split ~on:sep str' |> UList.to_tuple_exn in
    (target, fun x -> op ((get_cat (Char.of_string id)) x) (Int.of_string arg))

let is_rating_accepted ?(init_state = "in") ~tbl rating =
  let curr_state = ref init_state in
  while
    not
      (String.length !curr_state = 1
      && (Char.(!curr_state.[0] = 'A') || Char.(!curr_state.[0] = 'R')))
  do
    let rules = Hashtbl.find_exn tbl !curr_state in
    UList.repeat_until rules ~f:(fun (target, rule) ->
        if rule rating then (
          curr_state := target;
          true)
        else false)
  done;
  Char.(!curr_state.[0] = 'A')

let parse input =
  let tbl = Hashtbl.create (module String) in
  let workflows, ratings_raw =
    UString.split_on_substr ~substr:"\n\n" input
    |> List.map ~f:String.split_lines
    |> UList.to_tuple_exn
  in
  let ratings =
    List.map ratings_raw ~f:(fun rating ->
        String.drop_suffix (String.drop_prefix rating 1) 1
        |> String.split ~on:','
        |> List.map ~f:(fun rating ->
               let _, value =
                 String.split ~on:'=' rating |> UList.to_tuple_exn
               in
               Int.of_string value))
  in
  List.iter workflows ~f:(fun workflow ->
      let id, rest = String.split ~on:'{' workflow |> UList.to_tuple_exn in
      let rules = String.drop_suffix rest 1 |> String.split ~on:',' in
      Hashtbl.set tbl ~key:id ~data:(List.map rules ~f:parse_rule));

  (tbl, ratings)

let part1 (tbl, ratings) =
  List.filter ratings ~f:(is_rating_accepted ~tbl)
  |> List.fold_left ~init:0 ~f:(fun acc rating ->
         List.fold_left rating ~init:acc ~f:( + ))

let part2 (_, ratings) = List.length ratings

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day19 part1 - example data" =
  (parse >> part1)
    "px{a<2006:qkq,m>2090:A,rfg}\n\
     pv{a>1716:R,A}\n\
     lnx{m>1548:A,A}\n\
     rfg{s<537:gd,x>2440:R,A}\n\
     qs{s>3448:A,lnx}\n\
     qkq{x<1416:A,crn}\n\
     crn{x>2662:A,R}\n\
     in{s<1351:px,qqz}\n\
     qqz{s>2770:qs,m<1801:hdj,R}\n\
     gd{a>3333:R,R}\n\
     hdj{m>838:A,pv}\n\n\
     {x=787,m=2655,a=1222,s=2876}\n\
     {x=1679,m=44,a=2067,s=496}\n\
     {x=2036,m=264,a=79,s=2244}\n\
     {x=2461,m=1339,a=466,s=291}\n\
     {x=2127,m=1623,a=2188,s=1013}"
  = 19114

let%test "Day19 part2 - example data" =
  (parse >> part1)
    "px{a<2006:qkq,m>2090:A,rfg}\n\
     pv{a>1716:R,A}\n\
     lnx{m>1548:A,A}\n\
     rfg{s<537:gd,x>2440:R,A}\n\
     qs{s>3448:A,lnx}\n\
     qkq{x<1416:A,crn}\n\
     crn{x>2662:A,R}\n\
     in{s<1351:px,qqz}\n\
     qqz{s>2770:qs,m<1801:hdj,R}\n\
     gd{a>3333:R,R}\n\
     hdj{m>838:A,pv}\n\n\
     {x=787,m=2655,a=1222,s=2876}\n\
     {x=1679,m=44,a=2067,s=496}\n\
     {x=2036,m=264,a=79,s=2244}\n\
     {x=2461,m=1339,a=466,s=291}\n\
     {x=2127,m=1623,a=2188,s=1013}"
  = 167409079868000
