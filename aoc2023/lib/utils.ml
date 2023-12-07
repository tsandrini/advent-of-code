module List' = List
module String' = String
open Core


let uncons = function [] -> failwith "Utils.uncons: Empty list given" | hd :: tl -> (hd, tl)

let option_unwrap = function Some x -> x | None -> failwith "Utils.option_unwrap: None given"

let split_on_substring ~substring str =
  let rec aux acc str =
    match String.substr_index str ~pattern:substring with
    | None -> List.rev (str :: acc) (* No more occurrences of substring *)
    | Some index ->
      let before = String.sub str ~pos:0 ~len:index in
      let after =
        String.sub str
          ~pos:(index + String.length substring)
          ~len:(String.length str - index - String.length substring)
      in
      aux (before :: acc) after
  in
  aux [] str

let rec list_group_into_tuples = function
  | a :: b :: tail -> (a, b) :: list_group_into_tuples tail
  | [] -> []
  | _ -> failwith "Utils.list_group_into_tuples: List has an odd number of elements"


let list_to_triple = function
  | [ a; b; c ] -> (a, b, c)
  | _ -> failwith "Utils.list_to_triple: List does not have exactly three elements"

let list_to_tuple = function
  | [ a; b ] -> (a, b)
  | _ -> failwith "Utils.list_to_tuple: List does not have exactly two elements"


let reduce_multiple_whitespaces str =
  Str.global_replace (Str.regexp "[ \t\n\r]+") " " str

let elem_indices_of_list ~elem ~compare lst =
  let rec aux acc index = function
    | [] -> List.rev acc
    | hd :: tl ->
      if (compare hd elem) = 0 then aux (index :: acc) (index + 1) tl else aux acc (index + 1) tl
  in
  aux [] 0 lst

let rec rec_product_of_list l =
    let rec aux ~acc l1 l2 = match l1, l2 with
    | [], _ | _, [] -> acc
    | h1::t1, h2::t2 ->
        let acc = (h1::h2)::acc in
        let acc = (aux ~acc t1 l2) in
        aux ~acc [h1] t2
    (* now we can do the actual computation *)
    in match l with
    | [] -> []
    | [l1] -> List.map ~f:(fun x -> [x]) l1
    | l1::tl ->
        let tail_product = rec_product_of_list tl in
        aux ~acc:[] l1 tail_product
