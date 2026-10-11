#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <flint/flint.h>
#include <flint/arb.h>
#include <flint/arf.h>
#include <flint/fmpz.h>
static void set_input(arb_t result, slong lower_n, slong lower_e, slong upper_n, slong upper_e)
{
    arf_t lo, hi;
    arf_init(lo);
    arf_init(hi);
    arf_set_si_2exp_si(lo, lower_n, lower_e);
    arf_set_si_2exp_si(hi, upper_n, upper_e);
    arb_set_interval_arf(result, lo, hi, 256);
    arf_clear(lo);
    arf_clear(hi);
}
int main(int argc, char **argv)
{
    FILE *in;
    char line[2048], op[16];
    slong p, a, ae, b, be, c, ce, d, de;
    size_t line_no = 0, emitted = 0;
    arb_t left, right, out;
    fmpz_t low, high, exp;
    if (argc != 2 || strcmp(FLINT_VERSION, "3.0.1") != 0)
        return 64;
    in = fopen(argv[1], "r");
    if (!in) { perror("open corpus"); return 65; }
    arb_init(left);
    arb_init(right);
    arb_init(out);
    fmpz_init(low);
    fmpz_init(high);
    fmpz_init(exp);
    fprintf(stderr, "FLINT_VERSION=%s\n", FLINT_VERSION);
    fprintf(stderr, "ARB_REFERENCE_BITS=256\n");
    while (fgets(line, sizeof(line), in))
    {
        char *low_str, *high_str, *exp_str;
        int count;
        ++line_no;
        if (line[0] == '#' || line[0] == '\n') continue;
        count = sscanf(line, "%15s %ld %ld %ld %ld %ld %ld %ld %ld %ld",
                       op, &p, &a, &ae, &b, &be, &c, &ce, &d, &de);
        if (count != 10) {fprintf(stderr, "bad vector %zu\n", line_no); return 66;}
        if (p < 2 || p > 128) return 67;
        set_input(left, a, ae, b, be);
        set_input(right, c, ce, d, de);
        if (strcmp(op, "add") == 0) arb_add(out, left, right, 256);
        else if (strcmp(op, "sub") == 0) arb_sub(out, left, right, 256);
        else if (strcmp(op, "mul") == 0) arb_mul(out, left, right, 256);
        else if (strcmp(op, "div") == 0) arb_div(out, left, right, 256);
        else if (strcmp(op, "recip") == 0) arb_inv(out, left, 256);
        else if (strcmp(op, "neg") == 0) arb_neg(out, left);
        else {fprintf(stderr,"unknown op on line %zu\n",line_no);return 68;}
        if (!arb_is_finite(out)) {printf("%zu\t%s\tUNSUPPORTED_NONFINITE\n",line_no,op);++emitted;continue;}
        arb_get_interval_fmpz_2exp(low, high, exp, out);
        low_str = fmpz_get_str(NULL, 10, low);
        high_str = fmpz_get_str(NULL, 10, high);
        exp_str = fmpz_get_str(NULL, 10, exp);
        if (!low_str || !high_str || !exp_str) return 70;
        printf("%zu\t%s\t%s\t%s\t%s\t%s\n",
            line_no, op, low_str, exp_str, high_str, exp_str);
        flint_free(low_str);
        flint_free(high_str);
        flint_free(exp_str);
        ++emitted;
    }
    fclose(in);
    fmpz_clear(low);
    fmpz_clear(high);
    fmpz_clear(exp);
    arb_clear(left);
    arb_clear(right);
    arb_clear(out);
    fprintf(stderr, "FLINT_ARB_EVALUATED=%zu\n", emitted);
    return 0;
}
