module npu #(
    parameter int N = 2
) (
    input logic clk,
    input logic rst,
    input logic start,
    output logic done,
    output logic signed [31:0] result

);
  typedef enum logic {
    IDLE = 0,
    BUSY = 1
  } state_t;
  state_t state;

  logic signed [31:0] b[1];
  logic signed [31:0] w[N];
  logic signed [31:0] a[N];

  logic unsigned [$clog2(N):0] i;

  logic signed [63:0] pe_acc;
  pe u_pe (
      .clk        (clk),
      .rst        (rst),
      .init       (i == 0),
      .en         (state == BUSY),
      .a          (a[i]),
      .w          (w[i]),
      .accumulator(pe_acc)
  );

  logic signed [63:0] sum64;
  logic signed [31:0] sum32;

  assign sum64 = pe_acc + 64'(b[0]);

  always_comb begin
    if (sum64 == 64'(signed'(sum64[31:0]))) begin
      sum32 = sum64[31:0];
    end else begin
      sum32 = sum64[63] ? 32'sh8000_0000 : 32'sh7FFF_FFFF;
    end
  end

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      i <= 0;
      result <= 0;
      done <= 0;
      state <= IDLE;
    end else begin
      unique case (state)
        IDLE: begin
          if (start) begin
            done <= 0;
            state <= BUSY;
            i <= 0;
          end
        end
        BUSY: begin
          if (32'(i) == N) begin
            result <= sum32;
            done <= 1;
            state <= IDLE;
            i <= 0;
          end else begin
            i <= i + 1;
          end

        end
      endcase

    end
  end
endmodule
