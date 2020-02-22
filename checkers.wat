(module
  
  ;; Imports.
  ;; NOTE: Import must occur before all other declarations.
  (import "events" "notifyPieceCrowned"
    (func $host_notifyPieceCrowned (param $pieceX i32) (param $pieceY i32)))
  (import "events" "notifyPieceMoved"
    (func $host_notifyPieceMoved (param $pieceX i32) (param $pieceY i32)))


  ;; Exports.
  (export "indexForPosition" (func $indexForPosition))
  (export "offsetForPosition" (func $offsetForPosition))
  (export "isCrowned" (func $isCrowned))
  (export "isWhite" (func $isWhite))
  (export "isBlack" (func $isBlack))
  (export "withCrown" (func $withCrown))
  (export "withoutCrown" (func $withoutCrown))
  (export "memory" (memory $mem))


  ;; Declare memory usage. Indicates that $mem has one 64KB page of memory.
  (memory $mem 1)


  ;; Global constants. Properties for white color, black color, and crowned state.
  (global $WHITE i32 (i32.const 2))
  (global $BLACK i32 (i32.const 1))
  (global $CROWN i32 (i32.const 4))


  ;; Global variable
  (global $currentTurn (mut i32) (i32.const 0))


  ;; Calculates the index.
  ;; index = 8 * y + x
  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
    (i32.add
      (i32.mul
        (i32.const 8)
        (get_local $y)
      )
    (get_local $x)
    )
  )
  

  ;; Calculates the offset.
  ;; offset = (x + y * 8) * 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
    (i32.mul
      (call $indexForPosition (get_local $x) (get_local $y))
      (i32.const 4)
    )
  )


  ;; Determine if a piece is crowned.
  (func $isCrowned (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $CROWN))
      (get_global $CROWN)
    )
  )


  ;; Determine if a piece is white.
  (func $isWhite (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $WHITE))
      (get_global $WHITE)
    )
  )
  
  
  ;; Determine if a piece is black.
  (func $isBlack (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $BLACK))
      (get_global $BLACK)
    )
  )


  ;; Adds a crown to a given piece without mutation.
  (func $withCrown (param $piece i32) (result i32)
    (i32.or (get_local $piece) (get_global $CROWN))
  )


  ;; Removes a crown from a given piece without mutation.
  (func $withoutCrown (param $piece i32) (result i32)
    (i32.and (get_local $piece) (i32.const 3))
  )


  ;; Sets a piece on the board
  (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
    (i32.store
      (call $offsetForPosition
        (get_local $x)
        (get_local $y)
      )
      (get_local $piece)
    )
  )

  
  ;; Piece is within range of the board's linear memory space.
  (func $inRange (param $low i32) (param $high i32) (param $value i32) (result i32)
    (i32.and
      (i32.ge_s (get_local $value) (get_local $low))
      (i32.le_s (get_local $value) (get_local $high))
    )
  )


  ;; Gets a piece from a board. Out of range causes a trap.
  (func $getPiece (param $x i32) (param $y i32) (result i32)
    (if (result i32)
      (block (result i32)
        (i32.and
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (get_local $x)
          )
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (get_local $y)
          )
        )
      )
      (then
        (i32.load
          (call $offsetForPosition
            (get_local $x)
            (get_local $y)
          )
        )
      )
      (else
        (unreachable)
      )
    )    
  )


  ;; Get the current turn owner from global state.
  (func $getTurnOwner (result i32)
    (get_global $currentTurn)
  )


  ;; Get the current turn owner from global state.
  (func $setTurnOwner (param $piece i32)
    (set_global $currentTurn (get_local $piece))
  )


  ;; Toggle the turn owner to the opposite of its current value.
  (func $toggleTurnOwner
    (if (i32.eq (call $getTurnOwner) (i32.const 1))
      (then (call $setTurnOwner (i32.const 2)))
      (else (call $setTurnOwner (i32.const 1)))
    )
  )


  ;; Determine which player's turn it is.
  (func $isPlayersTurn (param $player i32) (result i32)
    (i32.gt_s
      (i32.and (get_local $player) (call $getTurnOwner))
      (i32.const 0)
    )
  )


  ;; Assess whether a piece should be crowned, based on the vertical index.
  ;; Black pieces are crowned in row 0; white pieces in row 7.
  ;; TODO: Is specific logic for 0/black and 7/white the best approach.
  (func $shouldCrown (param $pieceY i32) (param $piece i32) (result i32)
    (i32.or
      (i32.and
        (i32.eq
          (get_local $pieceY)
          (i32.const 0)
        )
        (call $isBlack (get_local $piece))
      )
      (i32.and
        (i32.eq
          (get_local $pieceY)
          (i32.const 7)
        )
        (call $isWhite (get_local $piece))
      )
    )
  )


  ;; Crown piece
  (func $crownPiece (param $x i32) (param $y i32)
    (local $piece i32)
    (set_local $piece (call $getPiece (get_local $x) (get_local $y)))
    (call $setPiece (get_local $x) (get_local $y)
      (call $withCrown (get_local $piece)))
    
    (call $host_notifyPieceCrowned (get_local $x) (get_local $y))
  )


  ;; Calculate distance (a.k.a difference) between two 1-d points.
  (func $distance (param $a i32) (param $b i32) (result i32)
    (i32.sub (get_local $a) (get_local $b))
  )


  

)
