open Core
open Utils

let count_perms_memo =
  let count_perms self (line, groups) =
    let len = String.length line in
    let groups_len = List.length groups in
    if len = 0 then if groups_len = 0 then 1 else 0
    else if groups_len = 0 then if String.contains line '#' then 0 else 1
    else if len < UList.fold_sum groups + groups_len - 1 then 0
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
  UMemo.memo_rec count_perms

let parse =
  String.split_lines
  >> List.map ~f:(fun line ->
         let spring_list, groups =
           String.split ~on:' ' line |> UList.to_tuple_exn
         in
         let groups =
           String.split ~on:',' groups |> List.map ~f:Int.of_string
         in
         (spring_list, groups))

let part1 =
  List.fold_left ~init:0 ~f:(fun acc (spring_list, groups) ->
      acc + count_perms_memo (spring_list, groups))

let part2 =
  List.fold_left ~init:0 ~f:(fun acc (spring_list, groups) ->
      acc
      + count_perms_memo
          ( List.init 5 ~f:(fun _ -> spring_list) |> String.concat ~sep:"?",
            List.init 5 ~f:(fun _ -> groups) |> List.concat ))

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day12 part1 - example data" =
  (parse >> part1)
    "???.### 1,1,3\n\
     .??..??...?##. 1,1,3\n\
     ?#?#?#?#?#?#?#? 1,3,1,6\n\
     ????.#...#... 4,1,1\n\
     ????.######..#####. 1,6,5\n\
     ?###???????? 3,2,1"
  = 21

let%test "Day12 part2 - example data" =
  (parse >> part2)
    "???.### 1,1,3\n\
     .??..??...?##. 1,1,3\n\
     ?#?#?#?#?#?#?#? 1,3,1,6\n\
     ????.#...#... 4,1,1\n\
     ????.######..#####. 1,6,5\n\
     ?###???????? 3,2,1"
  = 525152
