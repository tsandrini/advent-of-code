open Core

let parse input =
  String.split_lines input
  |> List.map ~f:(fun line ->
         let spring_list, groups =
           String.split ~on:' ' line |> Utils.list_to_tuple
         in
         let groups =
           String.split ~on:',' groups |> List.map ~f:Int.of_string
         in
         (spring_list, groups))

let count_perms_memo =
  let count_perms self (line, groups) =
    let len = String.length line in
    let groups_len = List.length groups in
    if len = 0 then if groups_len = 0 then 1 else 0
    else if groups_len = 0 then if String.contains line '#' then 0 else 1
    else if len < Utils.sum groups + groups_len - 1 then 0
    else
      match line.[0] with
      | '.' -> self (String.drop_prefix line 1, groups)
      | '#' -> (
          match groups with
          | [] -> assert false (*TODO should be unreachable?*)
          | group :: tl ->
              if String.contains (String.sub line ~pos:0 ~len:group) '.' then 0
              else if len > group && Char.(line.[group] = '#') then 0
              else self (String.drop_prefix line (group + 1), tl))
      | _ ->
          self ("#" ^ String.drop_prefix line 1, groups)
          + self ("." ^ String.drop_prefix line 1, groups)
  in
  Utils.memo_rec count_perms

let solve lines =
  let total_perms = ref 0 in
  List.iter lines ~f:(fun (spring_list, groups) ->
      total_perms := !total_perms + count_perms_memo (spring_list, groups));
  let part1 =
    List.fold_left lines ~init:0 ~f:(fun acc (spring_list, groups) ->
        acc + count_perms_memo (spring_list, groups))
  in
  let part2 =
    List.fold_left lines ~init:0 ~f:(fun acc (spring_list, groups) ->
        acc
        + count_perms_memo
            ( List.init 5 ~f:(fun _ -> spring_list) |> String.concat ~sep:"?",
              List.init 5 ~f:(fun _ -> groups) |> List.concat ))
  in
  Printf.printf "Part1 with nfold=0: %d\n" part1;
  Printf.printf "Part1 with nfold=5: %d\n" part2

let main input = In_channel.read_all input |> parse |> solve
