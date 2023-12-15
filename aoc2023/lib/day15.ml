open Core

let hash str =
  String.to_list str
  |> List.fold_left ~init:0 ~f:(fun acc c -> 17 * (acc + Char.to_int c) mod 256)

let parse input = String.strip input |> String.split ~on:','

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
          String.split seq ~on:'=' |> Utils.list_to_tuple
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

let solve seqs =
  let part1 = List.map seqs ~f:hash |> Utils.sum in
  let part2 = focusing_power_of_conf seqs in
  Printf.printf "Part 1: %d\n" part1;
  Printf.printf "Part 2: %d\n" part2

let main input = In_channel.read_all input |> parse |> solve
