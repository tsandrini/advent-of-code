open Core
open Utils

let nth_flip = flip List.nth_exn

type iter_node = {
  id : string;
  xmin : int;
  xmax : int;
  mmin : int;
  mmax : int;
  amin : int;
  amax : int;
  smin : int;
  smax : int;
}

let get_cat = function
  | 'x' -> nth_flip 0
  | 'm' -> nth_flip 1
  | 'a' -> nth_flip 2
  | 's' -> nth_flip 3
  | _ -> failwith "Invalid category"

let parse_rule str =
  if not (String.contains str ':') then (str, (str, ' ', '=', 0), fun _ -> true)
  else
    let str', target = String.split ~on:':' str |> UList.to_tuple_exn in
    let sep = if String.contains str' '<' then '<' else '>' in
    let op = if String.contains str' '<' then ( < ) else ( > ) in
    let id, arg = String.split ~on:sep str' |> UList.to_tuple_exn in
    ( target,
      (target, Char.of_string id, sep, Int.of_string arg),
      fun x -> op ((get_cat (Char.of_string id)) x) (Int.of_string arg) )

let is_rating_accepted ?(init_state = "in") ~tbl rating =
  let curr_state = ref init_state in
  while
    not
      (String.length !curr_state = 1
      && (Char.(!curr_state.[0] = 'A') || Char.(!curr_state.[0] = 'R')))
  do
    let rules = Hashtbl.find_exn tbl !curr_state in
    UList.repeat_until rules ~f:(fun (target, _, rule) ->
        if rule rating then (
          curr_state := target;
          true)
        else false)
  done;
  Char.(!curr_state.[0] = 'A')

let check_node_bounds node =
  node.xmin <= node.xmax && node.mmin <= node.mmax && node.amin <= node.amax
  && node.smin <= node.smax

let volume_of_node node =
  (node.xmax - node.xmin + 1)
  * (node.mmax - node.mmin + 1)
  * (node.amax - node.amin + 1)
  * (node.smax - node.smin + 1)

let transform_node ~spec:(target, id, op, arg) node =
  let f min' max' =
    match op with
    | '<' -> (min', min max' (arg - 1))
    | '>' -> (max min' (arg + 1), max')
    | 'l' -> (min', min max' arg)
    | 'g' -> (max min' arg, max')
    | _ -> failwith "Invalid operator"
  in
  if Char.(op = '=') then { node with id = target }
  else
    match id with
    | 'x' ->
        let min', max' = f node.xmin node.xmax in
        { node with id = target; xmin = min'; xmax = max' }
    | 'm' ->
        let min', max' = f node.mmin node.mmax in
        { node with id = target; mmin = min'; mmax = max' }
    | 'a' ->
        let min', max' = f node.amin node.amax in
        { node with id = target; amin = min'; amax = max' }
    | 's' ->
        let min', max' = f node.smin node.smax in
        { node with id = target; smin = min'; smax = max' }
    | _ -> failwith "Invalid category"

let volume_of_accepted_subset ?(init_state = "in") ~tbl =
  let vol = ref 0 in
  let queue = Linked_queue.create () in
  let curr_node =
    ref
      {
        id = init_state;
        xmin = 1;
        xmax = 4000;
        mmin = 1;
        mmax = 4000;
        amin = 1;
        amax = 4000;
        smin = 1;
        smax = 4000;
      }
  in
  Linked_queue.enqueue queue !curr_node;
  while not (Linked_queue.is_empty queue) do
    let node = Linked_queue.dequeue_exn queue in
    (*Skip node if invalid bounds*)
    if check_node_bounds node then (
      if
        String.length node.id = 1
        && (Char.(node.id.[0] = 'R') || Char.(node.id.[0] = 'A'))
      then
        vol :=
          !vol + if Char.(node.id.[0] = 'A') then volume_of_node node else 0
      else
        let rules = Hashtbl.find_exn tbl node.id in
        curr_node := node;
        List.iter rules ~f:(fun (_, spec, _) ->
            let target, id, op, arg = spec in
            Linked_queue.enqueue queue (transform_node ~spec !curr_node);

            if Char.(op <> '=') then
              curr_node :=
                transform_node
                  ~spec:(target, id, (if Char.(op = '>') then 'l' else 'g'), arg)
                  !curr_node))
  done;
  !vol

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

let part2 (tbl, _) = volume_of_accepted_subset ~init_state:"in" ~tbl

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
