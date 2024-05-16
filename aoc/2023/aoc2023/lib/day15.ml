open Core
open Utils

let hash =
  String.to_list >> List.fold_left ~init:0 ~f:(fun acc c -> 17 * (acc + Char.to_int c) mod 256)

let map_list_append_or_rewrite ~tbl ~key (str_id, focal_len) =
  let lst = Hashtbl.find_or_add tbl key ~default:(fun () -> []) in
  if
    List.mem lst (str_id, focal_len) ~equal:(fun (str_id1, _) (str_id2, _) ->
        String.(str_id1 = str_id2))
  then
    Hashtbl.set tbl ~key
      ~data:
        (List.map lst ~f:(fun (str_id', focal_len') ->
             if String.(str_id = str_id') then (str_id, focal_len)
             else (str_id', focal_len')))
  else Hashtbl.set tbl ~key ~data:(List.append lst [ (str_id, focal_len) ])

let map_list_remove ~tbl ~key ~str_id =
  let lst = Hashtbl.find_or_add tbl key ~default:(fun () -> []) in
  Hashtbl.set tbl ~key
    ~data:(List.filter lst ~f:(fun (str_id', _) -> String.(str_id <> str_id')))

let focusing_power_of_conf seqs =
  let map = Hashtbl.create (module Int) in

  List.iter seqs ~f:(fun seq ->
      if String.contains seq '=' then
        let str_id, focal_len =
          String.split seq ~on:'=' |> UList.to_tuple_exn
        in
        let label = hash str_id in
        map_list_append_or_rewrite ~tbl:map ~key:label
          (str_id, Int.of_string focal_len)
      else
        let str_id = String.drop_suffix seq 1 in
        let label = hash str_id in
        map_list_remove ~tbl:map ~key:label ~str_id);

  Hashtbl.fold map ~init:0 ~f:(fun ~key ~data acc ->
      acc
      + (key + 1)
        * List.foldi data ~init:0 ~f:(fun i acc' (_, focal_len) ->
              acc' + (focal_len * (i + 1))))

let parse = String.strip >> String.split ~on:','
let part1 = List.map ~f:hash >> UList.fold_sum
let part2 = focusing_power_of_conf

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day15 part1 - example data" =
  (parse >> part1) "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7" = 1320

let%test "Day15 part2 - example data" =
  (parse >> part2) "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7" = 145
