open Core
open Utils

type category =
  | Seed
  | Soil
  | Fertilizer
  | Water
  | Light
  | Temperature
  | Humidity
  | Location
[@@deriving sexp, compare, hash]

let category_of_string = function
  | "seed" -> Seed
  | "soil" -> Soil
  | "fertilizer" -> Fertilizer
  | "water" -> Water
  | "light" -> Light
  | "temperature" -> Temperature
  | "humidity" -> Humidity
  | "location" -> Location
  | _ -> failwith "invalid category"

let category_seq_of_seeds =
  [ Seed; Soil; Fertilizer; Water; Light; Temperature; Humidity; Location ]

let seed_category_path =
  List.map2_exn
    ~f:(fun a b -> (a, b))
    (List.slice category_seq_of_seeds 0 (List.length category_seq_of_seeds - 1))
    (List.slice category_seq_of_seeds 1 (List.length category_seq_of_seeds))

module CategoryPair = struct
  type t = category * category [@@deriving sexp, compare, hash]
end

let append_to_array table key element =
  let current_array = Hashtbl.find_or_add table key ~default:(fun () -> [||]) in
  let new_array = Array.append current_array [| element |] in
  Hashtbl.set table ~key ~data:new_array

let location_of_seed maps seed =
  let curr_id = ref seed in
  List.iter seed_category_path ~f:(fun curr_ptr ->
      let old_id = !curr_id in
      let curr_map = Hashtbl.find maps curr_ptr |> option_unwrap in
      let curr_map_len = Array.length curr_map in
      let i = ref 0 in
      while !curr_id = old_id && !i < curr_map_len do
        let target_rng, source_rng, len = curr_map.(!i) in
        if !curr_id >= source_rng && !curr_id <= source_rng + len then
          curr_id := target_rng + (!curr_id - source_rng);

        i := !i + 1
      done
      (* If ID hasn't been found it stays the same -> hence we do exactly nothing *));
  !curr_id

let solve lines =
  let line, lines = uncons lines in
  let seeds =
    line |> String.split ~on:':' |> List.tl_exn |> List.hd_exn |> String.strip
    |> String.split ~on:' ' |> List.map ~f:Int.of_string
    |> UList.group_into_tuples
  in
  let maps = Hashtbl.create (module CategoryPair) in
  let map_ptr = ref (Seed, Soil) in

  (* First we parse the maps *)
  List.iter lines ~f:(fun line ->
      if not (String.is_empty line) then
        if String.is_substring ~substring:"map" line then
          let ids =
            line |> String.split ~on:' ' |> List.hd_exn
            |> UString.split_on_substr ~substr:"-to-"
          in
          map_ptr := List.map ~f:category_of_string ids |> UList.to_tuple_exn
        else
          let spec =
            line |> String.split ~on:' ' |> List.map ~f:Int.of_string
          in
          append_to_array maps !map_ptr (UList.to_triple_exn spec));

  (*Binary search -- this is only possible due to the fact that we are working *)
  (* with linear spaces (ranges) that are inherently ordered *)
  let rec aux left right =
    let left_loc = location_of_seed maps left in
    let right_loc = location_of_seed maps right in
    let range_len = right - left in
    if range_len = 1 then [ left_loc; right_loc ]
    else if left_loc + range_len = right_loc then [ left_loc ]
    else
      let mid = range_len / 2 in
      List.concat [ aux left (left + mid); aux (left + mid) right ]
  in

  (*Init the traversal and find the minimum*)
  List.map seeds ~f:(fun (range_start, range_len) ->
      let left = range_start in
      let right = range_start + range_len in
      aux left right)
  |> List.concat
  |> List.fold_left ~f:min ~init:(location_of_seed maps (List.hd_exn seeds |> fst))

let main input = In_channel.read_lines input |> solve |> Printf.printf "%d\n"

let%test "day05pat1 - test example input" =
  solve
    [
      "seeds: 79 14 55 13";
      "";
      "seed-to-soil map:";
      "50 98 2";
      "52 50 48";
      "";
      "soil-to-fertilizer map:";
      "0 15 37";
      "37 52 2";
      "39 0 15";
      "";
      "fertilizer-to-water map:";
      "49 53 8";
      "0 11 42";
      "42 0 7";
      "57 7 4";
      "";
      "water-to-light map:";
      "88 18 7";
      "18 25 70";
      "";
      "light-to-temperature map:";
      "45 77 23";
      "81 45 19";
      "68 64 13";
      "";
      "temperature-to-humidity map:";
      "0 69 1";
      "1 0 69";
      "";
      "humidity-to-location map:";
      "60 56 37";
      "56 93 4";
    ]
  = 46
