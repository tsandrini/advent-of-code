open Core

(*Base function & operators that should be brought into the scope by default*)
let id = Fn.id
let apply_n_times = Fn.apply_n_times
let flip = Fn.flip
let const = Fn.const
let (<<) f g x = f (g x)
let (>>) f g x = g (f x)

let uncons = function
  | [] -> failwith "Utils.uncons: Empty list given"
  | hd :: tl -> (hd, tl)

let option_unwrap = function
  | Some x -> x
  | None -> failwith "Utils.option_unwrap: None given"

module UList : sig
  type 'a t = 'a list

  val to_tuple : 'a t -> ('a * 'a) option
  val to_tuple_exn : 'a t -> 'a * 'a
  val to_triple : 'a t -> ('a * 'a * 'a) option
  val to_triple_exn : 'a t -> 'a * 'a * 'a
  val group_into_tuples : 'a t -> ('a * 'a) t
  val elem_indices_of_list : cmp:('a -> 'a -> int) -> elem:'a -> 'a t -> int t
  val repeat_until : f:('a -> bool) -> 'a t -> unit
  val repeati_until : f:(int -> 'a -> bool) -> 'a t -> unit
  val fold_sum : int t -> int
  val fold_product : int t -> int
  val fold_all_true : bool t -> bool
  val fold_any_true : bool t -> bool
  val cartesian_product_rec : 'a t t -> 'a t t
end = struct
  type 'a t = 'a list = [] | ( :: ) of 'a * 'a list

  let to_tuple = function [ a; b ] -> Some (a, b) | _ -> None

  let to_tuple_exn = function
    | [ a; b ] -> (a, b)
    | _ ->
        failwith "UList.to_tuple_exn: List does not have exactly two elements"

  let to_triple = function [ a; b; c ] -> Some (a, b, c) | _ -> None

  let to_triple_exn = function
    | [ a; b; c ] -> (a, b, c)
    | _ ->
        failwith
          "UList.to_triple_exn: List does not have exactly three elements"

  let rec group_into_tuples = function
    | [] -> []
    | [ _ ] -> []
    | a :: b :: tl -> (a, b) :: group_into_tuples tl

  let elem_indices_of_list ~cmp ~elem lst =
    let rec aux acc index = function
      | [] -> acc
      | hd :: tl ->
          if cmp hd elem = 0 then aux (index :: acc) (index + 1) tl
          else aux acc (index + 1) tl
    in
    aux [] 0 lst |> List.rev

  let repeat_until ~f lst =
    let rec aux = function
      | [] -> aux lst
      | hd :: tl -> if Bool.(f hd <> true) then aux tl
    in
    aux lst

  let repeati_until ~f lst =
    let rec aux idx = function
      | [] -> aux idx lst
      | hd :: tl -> if Bool.(f idx hd <> true) then aux (idx + 1) tl
    in
    aux 0 lst

  let fold_sum = List.fold ~init:0 ~f:( + )
  let fold_product = List.fold ~init:1 ~f:( * )
  let fold_all_true = List.fold ~init:true ~f:( && )
  let fold_any_true = List.fold ~init:false ~f:( || )

  let rec cartesian_product_rec l =
    let rec aux ~acc l1 l2 =
      match (l1, l2) with
      | [], _ | _, [] -> acc
      | h1 :: t1, h2 :: t2 ->
          let acc = (h1 :: h2) :: acc in
          let acc = aux ~acc t1 l2 in
          aux ~acc [ h1 ] t2
      (* now we can do the actual computation *)
    in
    match l with
    | [] -> []
    | [ l1 ] -> List.map ~f:(fun x -> [ x ]) l1
    | l1 :: tl ->
        let tail_product = cartesian_product_rec tl in
        aux ~acc:[] l1 tail_product
end

module UMat : sig
  type 'a t = 'a list list

  val transpose : 'a t -> 'a t
  val rot_90ccw : 'a t -> 'a t
  val rot_90cw : 'a t -> 'a t
  val rot_180 : 'a t -> 'a t
end = struct
  type 'a t = 'a list list

  let rec transpose = function
    | [] | [] :: _ -> []
    | matrix ->
        List.map ~f:List.hd_exn matrix
        :: transpose (List.map ~f:List.tl_exn matrix)

  let rot_90ccw mat = transpose (List.map ~f:List.rev mat)
  let rot_90cw mat = List.map ~f:List.rev (transpose mat)
  let rot_180 mat = List.rev (List.map ~f:List.rev mat)
end

module UString : sig
  type t = string

  val split_on_substr : substr:string -> t -> t list
  val reduce_multiple_whitespaces : t -> t
end = struct
  type t = string

  let split_on_substr ~substr str =
    let rec aux acc str =
      match String.substr_index str ~pattern:substr with
      | None -> List.rev (str :: acc) (* No more occurrences of substring *)
      | Some index ->
          let before = String.sub str ~pos:0 ~len:index in
          let after =
            String.sub str
              ~pos:(index + String.length substr)
              ~len:(String.length str - index - String.length substr)
          in
          aux (before :: acc) after
    in
    aux [] str

  let reduce_multiple_whitespaces str =
    Str.global_replace (Str.regexp "[ \t\n\r]+") " " str
end

module UMath : sig
  val gcd : int -> int -> int
  val lcm : int -> int -> int
end = struct
  let rec gcd a b = if b = 0 then a else gcd b (a mod b)
  let lcm a b = a * b / gcd a b
end

module UMemo : sig
  val memo : ('a -> 'b) -> 'a -> 'b
  val memo_rec : (('a -> 'b) -> 'a -> 'b) -> 'a -> 'b
end = struct
  let memo f =
    let h = Hashtbl.Poly.create () ~size:11 in
    fun x ->
      match Hashtbl.find h x with
      | Some y -> y
      | None ->
          let y = f x in
          Hashtbl.set h ~key:x ~data:y;
          y

  let memo_rec f =
    let h = Hashtbl.Poly.create () ~size:16 in
    let rec g x =
      match Hashtbl.find h x with
      | Some y -> y
      | None ->
          let y = f g x in
          Hashtbl.set h ~key:x ~data:y;
          y
    in
    g
end
