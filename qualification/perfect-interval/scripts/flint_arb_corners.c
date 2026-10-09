/* Isolated FLINT/Arb 3.0.1 point-corner reference.
 * For finite rectangular arithmetic without denominator zero, the extrema
 * are attained at endpoint corners. Every corner is evaluated as two exact
 * point Arb balls. Python separately proves the corner-extrema property.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <flint/flint.h>
#include <flint/arb.h>
#include <flint/fmpz.h>

static void point(arb_t value, slong n, slong exponent)
{
    arb_set_si(value, n);
    arb_mul_2exp_si(value, value, exponent);
}
int main(int argc, char **argv)
{
    FILE *input;
    char line[2048], op[16];
    slong p, an, ae, bn, be, cn, ce, dn, de;
    size_t line_no = 0, emitted = 0;
    arb_t x, y, result;
    fmpz_t low, high, exp;
    if (argc != 2 || strcmp(FLINT_VERSION, "3.0.1") != 0) return 64;
    input = fopen(argv[1], "r");
    if (!input) { perror("open"); return 65; }
    arb_init(x); arb_init(y); arb_init(result);
    fmpz_init(low); fmpz_init(high); fmpz_init(exp);
    fprintf(stderr, "FLINT_VERSION=%s ARB_BITS=256 POINT_CORNERS\n", FLINT_VERSION);
    while (fgets(line, sizeof(line), input))
    {
        int matched, corners, k;
        slong xnum[2], xexp[2], ynum[2], yexp[2];
        ++line_no;
        if (line[0]=='#' || line[0]=='\n') continue;
        matched = sscanf(line, "%15s %ld %ld %ld %ld %ld %ld %ld %ld %ld",
            op, &p, &an, &ae, &bn, &be, &cn, &ce, &dn, &de);
        if (matched != 10 || p < 2 || p > 128) return 66;
        xnum[0]=an; xnum[1]=bn; xexp[0]=ae; xexp[1]=be;
        ynum[0]=cn; ynum[1]=dn; yexp[0]=ce; yexp[1]=de;
        corners = (strcmp(op,"neg")==0 || strcmp(op,"recip")==0)?2:4;
        for (k=0;k<corners;k++)
        {
            char *lo_str, *hi_str, *exp_str;
            int xi=k/2, yi=k%2;
            if (corners==2) xi=k;
            point(x,xnum[xi],xexp[xi]);
            point(y,ynum[yi],yexp[yi]);
            if (strcmp(op,"add")==0) arb_add(result,x,y,256);
            else if (strcmp(op,"sub")==0) arb_sub(result,x,y,256);
            else if (strcmp(op,"mul")==0) arb_mul(result,x,y,256);
            else if (strcmp(op,"div")==0) arb_div(result,x,y,256);
            else if (strcmp(op,"recip")==0) arb_inv(result,x,256);
            else if (strcmp(op,"neg")==0) arb_neg(result,x);
            else return 67;
            if (!arb_is_finite(result)) {
                fprintf(stderr,"nonfinite corner row %zu corner %d\n",line_no,k);
                return 68;
            }
            arb_get_interval_fmpz_2exp(low,high,exp,result);
            lo_str=fmpz_get_str(NULL,10,low);
            hi_str=fmpz_get_str(NULL,10,high);
            exp_str=fmpz_get_str(NULL,10,exp);
            if (!lo_str || !hi_str || !exp_str) return 69;
            printf("%zu\t%s\t%d\t%s\t%s\t%s\t%s\n",
                line_no,op,k,lo_str,exp_str,hi_str,exp_str);
            flint_free(lo_str);
            flint_free(hi_str);
            flint_free(exp_str);
            emitted++;
        }
    }
    fclose(input);
    fmpz_clear(low); fmpz_clear(high); fmpz_clear(exp);
    arb_clear(x); arb_clear(y); arb_clear(result);
    fprintf(stderr,"ARB_CORNER_OUTPUTS=%zu\n",emitted);
    return 0;
}
