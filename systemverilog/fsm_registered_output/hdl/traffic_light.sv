module traffic_light (
  input logic clk,
  input logic reset,

  output  logic green,
  output  logic yellow,
  output  logic red
);

localparam GREEN_BIT            = 0,
           YELLOW_TO_GREEN_BIT  = 1,
           YELLOW_TO_RED_BIT    = 2,
           RED_BIT              = 3;

typedef enum logic [3:0] {GREEN             = (1 << GREEN_BIT),
                          YELLOW_TO_GREEN   = (1 << YELLOW_TO_GREEN_BIT),
                          YELLOW_TO_RED     = (1 << YELLOW_TO_RED_BIT),
                          RED               = (1 << RED_BIT)} state_t;

`define assert_value(value) \
  assert(^value !== 1'bx) \
  else $error("%m, value has undefined bit")

state_t cur_state, next_state;

always_ff @(posedge clk) begin : FSM_STATE_UPDATE
  `assert_value(reset);
  if (reset)
    cur_state <= RED;
  else
    cur_state <= next_state;
end

always_comb begin : FSM_STATE_TRANSITION
  unique case (1'b1)
    cur_state[RED_BIT]: begin
      next_state = YELLOW_TO_GREEN;
    end
    cur_state[YELLOW_TO_GREEN_BIT]: begin
      next_state = GREEN;
    end
    cur_state[GREEN_BIT]: begin
      next_state = YELLOW_TO_RED;
    end
    cur_state[YELLOW_TO_RED_BIT]: begin
      next_state = RED;
    end
    default:
      next_state = RED;
  endcase
end

always_ff @(posedge clk) begin : FSM_OUTPUT_LOGIC
  if (reset) begin
    red     <= 1'b1;
    yellow  <= 1'b0;
    green   <= 1'b0;
  end else begin
    red     <= 1'b0;
    yellow  <= 1'b0;
    green   <= 1'b0;

    case (1'b1)
      next_state[RED_BIT]: begin
        red <= 1'b1;
      end
      next_state[YELLOW_TO_GREEN_BIT]: begin
        yellow <= 1'b1;
      end
      next_state[GREEN_BIT]: begin
        green <= 1'b1;
      end
      next_state[YELLOW_TO_RED_BIT]: begin
        yellow <= 1'b1;
      end
      default: begin
        red <= 1'b1;
      end
    endcase
  end
end

endmodule
